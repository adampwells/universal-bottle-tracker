<template>
  <q-page>
    <q-list>
      <q-item>
        <q-item-section>
          <q-item-label>
            What is this thing?
          </q-item-label>
          <q-item-label caption>
            The Universal Bottle Tracker is a simple application to help brewers track what beer is in their bottles.      Each bottle gets a unique <a href="/labels">QR code label</a> and when the brewer is bottling their beer, each bottle gets scanned using a mobile phone and linked to the Brewfather batch.
            Later you can use the web application to scan the bottle to find out what is in it. The QR code has the web site embedded in it, so your friends can scan the code with their phone camera and
              get info on the beer.
          </q-item-label>
        </q-item-section>
      </q-item>
      <q-item>
        <q-item-section>
          <q-item-label>
            If my friend scans one of my bottles, do they get access to my Brewfather account?
          </q-item-label>
          <q-item-label caption lines="2">
            Absolutely not! The web app running on your phone is the only place the Brewfather credentials live, and only your device talks with Brewfather.
          </q-item-label>
        </q-item-section>
      </q-item>
      <q-item>
        <q-item-section>
          <q-item-label>
            Can someone just guess bottle identifiers and get info on what I am brewing?
          </q-item-label>
          <q-item-label caption lines="2">
            Good luck with that... the identifiers are globally unique and effectively un-guessable. Technically they are 128 bit globally unique identifiers using cryptographically strong random APIs
            that guarantees a proper distribution of symbols.
          </q-item-label>
        </q-item-section>
      </q-item>
    </q-list>
    <q-btn @click="downloadLabels" label="Download word file for Avery 21UP labels"/>
  </q-page>
</template>

<script>
import {defineComponent} from 'vue'

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
