<template>
  <q-page class="flex flex-center">
    <div class="q-pa-md">
      <h6>Brewfather API Key</h6>
      <p>These credentials are stored locally on this browser only.</p>
      <div class="row q-gutter-md">
        <div class="col-12">
          <q-input v-model="brewFatherId" label="Brewfather User ID" @blur="saveCredentials"/>
        </div>
        <div class="col-12">
          <q-input type="password" v-model="brewFatherKey" label="Brewfather API Key" @blur="saveCredentials"/>
        </div>
        <div class="col-12">
          <q-btn size="sm" label="Load Batches" @click="loadBatches"/>
        </div>
      </div>
      <h6>How do I get my User ID and API Key?</h6>
      <div class="row q-gutter-y-lg justify-around">
        <q-card class="col-8" bordered>
          <q-img src="/IMG_1658.PNG" fit="contain"/>
        </q-card>
        <q-card class="col-8" bordered>
          <q-img src="/IMG_1659.PNG" fit="contain"/>
        </q-card>
        <q-card class="col-8" bordered>
          <q-img src="/IMG_1660.PNG" fit="contain"/>
        </q-card>
        <q-card class="col-8" bordered>
          <q-img src="/IMG_1661.PNG" fit="contain"/>
        </q-card>
      </div>
    </div>
  </q-page>
</template>

<script setup>
import {onMounted, ref} from "vue";
import { useQuasar } from 'quasar'
import api from "src/js/brewfather.js"

let brewFatherKey = ref('')
let brewFatherId = ref('')

onMounted(() => {
      brewFatherKey.value = localStorage.getItem('brewfatherKey')
      brewFatherId.value = localStorage.getItem('brewfatherId')
      console.log('Loaded Brewfather Credentials' + brewFatherKey.value + ' ' + brewFatherId.value)
    }
)

const $q = useQuasar()

function showNotify (message, type) {
  $q.notify({
    message: message,
    type: type
  })
}

function saveCredentials() {
  console.log('Saving Brewfather Credentials')
  localStorage.setItem('brewfatherKey', brewFatherKey.value)
  localStorage.setItem('brewfatherId', brewFatherId.value)
}

async function loadBatches() {
  api.getConditioningBatches().then((data) => {
    showNotify('Loaded ' + data.length + ' batches', 'positive')
  }).catch((error) => {
    showNotify('Error loading batches', 'negative')
  })
}

</script>