<template>
  <q-page class="flex flex-center">
    <div class="q-pa-md">
      <h4>Brewfather API Key</h4>
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
      <h4>How do I get my User ID and API Key?</h4>
      <div class="row q-gutter-y-lg">
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

import api from "src/js/brewfather.js"

let brewFatherKey = ref('')
let brewFatherId = ref('')

onMounted(() => {
      brewFatherKey.value = localStorage.getItem('brewfatherKey')
      brewFatherId.value = localStorage.getItem('brewfatherId')
      console.log('Loaded Brewfather Credentials' + brewFatherKey.value + ' ' + brewFatherId.value)
    }
)

function saveCredentials() {
  console.log('Saving Brewfather Credentials')
  localStorage.setItem('brewfatherKey', brewFatherKey.value)
  localStorage.setItem('brewfatherId', brewFatherId.value)
}

async function loadBatches() {
  api.getConditioningBatches().then((data) => {
    console.log(data)
  })
}

</script>