<template>
  <q-page class="flex flex-center">
    <div>
      <div class="container-sm" style="max-width: 150px">
        <qrcode-stream @detect="onDetect"></qrcode-stream>
      </div>
      <q-card-section>
        <div class="row justify-center">
          <div class="col-12"  v-if="currentBottle">
            <q-select label="Choose Batch" :options="batches" v-model="currentBottle.batch_id" map-options emit-value @update:model-value="updateBottle"/>
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
    const selectedBatch = ref('empty bottle');
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
      await getOrCreateBottle(scannedBottleId.value)
    };

    const getOrCreateBottle = async (bottleId) => {
      let bottle = await api.getOrCreateBottle(bottleId)
      if (bottle) {
        currentBottle.value = bottle
      }
    }

    const updateBottle = async () => {
      await api.updateBottle(currentBottle.value)
    }

    onMounted(() => {
          brewFatherKey.value = localStorage.getItem('brewfatherKey')
          brewFatherId.value = localStorage.getItem('brewfatherId')
          if (brewFatherKey.value && brewFatherId.value) {
            api.getConditioningBatches().then((data) => {
              batches.value = data.map(bb => {
                return {
                  value: bb._id,
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
