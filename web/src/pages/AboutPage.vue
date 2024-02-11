<template>
  <q-page class="flex flex-center">
    <h4>What is this thing?</h4>
    <p>The Universal Bottle Tracker is a simple application to help brewers track what beer is in their bottles.</p>
    <p>Each bottle gets a unique <a href="/labels">QR code label</a> and when the brewer is bottling their beer, each bottle gets scanned using a mobile phone and linked to the Brewfather batch.</p>
    <p>Later you can use the web application to scan the bottle to find out what is in it. The QR code has the web site embedded in it, so your friends can scan the code with their phone camera and get info on the beer.</p>
    <q-btn @click="downloadLabels" label="Download word file for Avery 21UP labels"/>
  </q-page>
</template>

<script>
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'AboutPage',
  setup() {

    const downloadLabels = () => {
      fetch(`/labels`, {
        method: 'GET',
      })
          .then(response => response.blob())
          .then(blob => {

            let fileName = `bottle_labels_${new Date().getTime()}.docx`
            var url = window.URL.createObjectURL(blob)
            var a = document.createElement('a')
            a.href = url
            a.download = fileName
            document.body.appendChild(a)
            a.click()
            a.remove()
          })
    }

    return {
      downloadLabels
    }
  }
})
</script>
