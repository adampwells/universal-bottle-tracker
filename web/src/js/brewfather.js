var url = ''
if (process.env.DEV) {
    url = 'http://localhost:3000'
}
export default {

    async getConditioningBatches() {

        let brewfatherId = localStorage.getItem('brewfatherId');
        let brewfatherKey = localStorage.getItem('brewfatherKey');
        let headers = new Headers();
        headers.set('Authorization', 'Basic ' + window.btoa(brewfatherId + ":" + brewfatherKey));

        const response = await fetch('https://api.brewfather.app/v2/batches?status=Conditioning&limit=50&include=measuredAbv,bottlingDate&order_by=batchNo&order_by_direction=desc', {
            method:'GET',
            mode: 'cors',
            redirect: 'follow',
            headers: headers,
        })

        return response.json()

    },

    async getOrCreateBottle(bottleId) {

        const response = await fetch(`${url}/api/bottle/${bottleId}`, {
            method:'GET',
            mode: 'cors',
            redirect: 'follow',
        })

        return response.json()

    },

    async updateBottle(bottle) {

        const response = await fetch(`${url}/api/bottle`, {
            method:'PUT',
            mode: 'cors',
            redirect: 'follow',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(bottle),
        })

        return response.json()

    }

}
