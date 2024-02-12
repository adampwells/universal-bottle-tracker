<template>
  <q-page class="flex flex-center">
    <div>
      <div class="container-sm" style="max-width: 150px">
        <qrcode-stream @detect="onDetect"></qrcode-stream>
      </div>
      <q-card-section>
        <div class="row justify-center">
          <div class="col-12"  v-if="currentBottle">
            <q-select label="Choose Batch" :options="batches" v-model="selectedBatch" map-options emit-value @update:model-value="updateBottle" style="width: 200px"/>
          </div>
        </div>
      </q-card-section>
    </div>
  </q-page>
</template>

<script>
import {defineComponent, onMounted, ref} from 'vue'
import {useRoute, useRouter} from 'vue-router'
import { useQuasar} from "quasar";
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
    let brewFatherKey = ref('')
    let brewFatherId = ref('')

    const $q = useQuasar()

    function showNotify (message, type) {
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
      scannedBottleId.value = value[value.length - 1];
      console.log('Scanned Bottle ID: ' + scannedBottleId.value)
      await getOrCreateBottle(scannedBottleId.value)
    };

    const getOrCreateBottle = async (bottleId) => {
      let response = await api.getOrCreateBottle(bottleId)
      if (response.data) {
        currentBottle.value = response.data
        if (currentBottle.value.batch_id) {
          let value = batches.value.find(b => Number(b.value.batchNo) === Number(currentBottle.value.batch_id));
          console.log(`bottle has a batch [${value.label}]`)
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
              batches.value = data.map(bb => {
                return {
                  value: bb,
                  label: '#' + bb.batchNo + ' ' + bb.recipe.name + ' (' + bb.measuredAbv + '% ABV)',
                  batchNo: Number(bb.batchNo),
                }
              });
              batches.value.sort((a, b) => a.batchNo > b.batchNo ? -1 : a.batchNo < b.batchNo ? 1 : 0)
            })
          } else {
            console.log('No Brewfather Credentials' + brewFatherKey.value + ' ' + brewFatherId.value)
          }
          if (bottleId.value) {
            getOrCreateBottle(bottleId.value)
          }
        }
    )

    return {
      batches,
      bottleId,
      currentBottle,
      scannedBottleId,
      selectedBatch,
      onDetect,
      updateBottle
    }
  }
})
</script>
