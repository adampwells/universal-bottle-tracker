<template>
  <q-page class="flex flex-center">
    <div class="q-pa-md q-gutter-lg justify-center">
      <h6 class="justify-center">Scan a bottle QR code to get started</h6>
      <q-card v-if="currentBottle" class="bg-blue-1">
        <q-card-section>
          <div class="text-h6">{{ currentBottle.batch_name }}</div>
          <div class="text-subtitle2">{{ currentBottle.saved_by }}</div>
        </q-card-section>
        <q-card-section v-if="selectedBatch">
          <div class="text">Batch: {{ selectedBatch.batchNo }}</div>
          <div class="text">ABV: {{ selectedBatch.measuredAbv }}%</div>
          <div class="text">Conditioning for {{ selectedBatch.conditioning }}</div>
        </q-card-section>
      </q-card>
      <div class="row justify-center">
        <div class="container-sm col-10" style="max-width: 150px">
          <qrcode-stream @detect="onDetect"></qrcode-stream>
        </div>
      </div>
      <q-card-section>
        <div class="row justify-center">
          <div class="col-auto" v-if="currentBottle">
            <div class="text-secondary">Bottle Id {{ currentBottle.bottle_id }}</div>
          </div>
        </div>
        <div class="row justify-center">
          <div class="col-auto" v-if="currentBottle && validCredentials">
            <q-select label="Choose Batch" :options="batches" v-model="selectedBatch" map-options emit-value @update:model-value="updateBottle" style="width: 300px"/>
          </div>
        </div>
      </q-card-section>
    </div>
  </q-page>
</template>

<script>
import {defineComponent, onMounted, ref} from 'vue'
import {useRoute, useRouter} from 'vue-router'
import {useQuasar} from "quasar";
import {QrcodeStream} from "vue-qrcode-reader";
import api from "src/js/brewfather";

export default defineComponent({
  name: 'BottlePage',
  components: {
    QrcodeStream,
  },
  setup() {
    const router = useRouter()
    const route = useRoute()
    const bottleId = ref(route.params.bottleId)
    const scannedBottleId = ref(null);
    const batches = ref([])
    const empty = {
      batch_id: undefined,
      bottle_id: undefined,
      batch_name: undefined,
      batch_description: undefined,
      saved_by: undefined,
    }
    const currentBottle = ref(undefined);
    const selectedBatch = ref(undefined);
    const validCredentials = ref(false);
    let brewFatherKey = ref('')
    let brewFatherId = ref('')

    const $q = useQuasar()

    function showNotify(message, type) {
      $q.notify({
        message: message,
        type: type,
        timeout: 50
      })
    }

    const onDetect = async (result) => {
      let res = await result
      showNotify('Got it!', 'positive')
      let value = res[0].rawValue.split('/');
      if (value.length === 1) {
        scannedBottleId.value = value[0];
      } else {
        value = value[value.length - 1];
        if (value.indexOf('?b=') === -1) {
          scannedBottleId.value = value;
        } else {
          scannedBottleId.value = value.substring(value.indexOf('?b=') + 3, value.length);
        }
      }
      console.log('Scanned Bottle ID: ' + scannedBottleId.value)
      await getOrCreateBottle(scannedBottleId.value)
    };

    const getOrCreateBottle = async (bottleId) => {
      let response = await api.getOrCreateBottle(bottleId)
      if (response.data) {
        console.log('Got Bottle: ' + JSON.stringify(response.data))
        currentBottle.value = response.data
        if (response.data.batch_id) {
          let value = batches.value.find(b => Number(b.value.batchNo) === Number(response.data.batch_id));
          selectedBatch.value = value
        } else {
          console.log('bottle has no batch')
          selectedBatch.value = undefined
        }
      }
    }

    const updateBottle = async () => {
      currentBottle.value.batch_id = `${selectedBatch.value.batchNo}`
      currentBottle.value.saved_by = selectedBatch.value.brewer
      currentBottle.value.batch_name = '#' + selectedBatch.value.batchNo + ' ' + selectedBatch.value.recipe.name + ' (' + selectedBatch.value.measuredAbv + '% ABV)'
      await api.updateBottle(currentBottle.value).catch((error) => {
        console.log(JSON.stringify(error))
      })
    }

    onMounted(() => {
          brewFatherKey.value = localStorage.getItem('brewfatherKey')
          brewFatherId.value = localStorage.getItem('brewfatherId')
          if (brewFatherKey.value && brewFatherId.value) {
            api.getConditioningBatches().then((data) => {
              validCredentials.value = true
              batches.value = data.map(bb => {
                const bottlingDate = new Date(bb.bottlingDate)
                const conditioning = Math.trunc((new Date().getTime() - bb.bottlingDate) / (60 * 60 * 24 * 1000)) + ' days'
                bb.conditioning = conditioning
                console.log('Conditioning: ' + conditioning)
                return {
                  value: bb,
                  label: '#' + bb.batchNo + ' ' + bb.recipe.name + ' (' + bb.measuredAbv + '% ABV)',
                  batchNo: Number(bb.batchNo),
                  conditioning: conditioning
                }
              });
              batches.value.sort((a, b) => a.batchNo > b.batchNo ? -1 : a.batchNo < b.batchNo ? 1 : 0)
            })
          } else {
            console.log('No Brewfather Credentials' + brewFatherKey.value + ' ' + brewFatherId.value)
          }

          lookupBottleIdFromQueryParams()
        }
    )

    const lookupBottleIdFromQueryParams = async () => {
      router.isReady().then(() => {
        console.log('Query Params: ' + JSON.stringify(route.query))
        if (route.query && route.query.b) {
          bottleId.value = route.query.b
          getOrCreateBottle(bottleId.value)
        }
      })
    }

    return {
      batches,
      bottleId,
      currentBottle,
      scannedBottleId,
      selectedBatch,
      validCredentials,
      onDetect,
      updateBottle
    }
  }
})
</script>
