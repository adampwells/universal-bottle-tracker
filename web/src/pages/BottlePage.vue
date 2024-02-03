<template>
  <q-page class="flex flex-center">
    bottle [{{ bottleId}}]
    <div class="container-sm" style="max-width: 300px">
      <qrcode-stream @detect="onDetect"></qrcode-stream>
    </div>
    <div>
      <p>Scanned bottle id: {{ scannedBottleId }}</p>
    </div>
  </q-page>
</template>

<script>
import { defineComponent, ref } from 'vue'
import {useRoute, useRouter} from 'vue-router'
import {QrcodeStream} from "vue-qrcode-reader";

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

    const onDetect = async (result) => {
      console.log('onDetect', result)
      let res = await result
      scannedBottleId.value = res[0].rawValue;
    };

    return {
      bottleId,
      scannedBottleId,
      onDetect
    }
  }
})
</script>
