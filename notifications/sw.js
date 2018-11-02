const maincache = "rivers.run"

function handlefetch(event) {
  let url = event.request.url
  
  //USGS data is date dependent. We should therefore ignore the query parameter
  url = url.slice(0, url.indexOf("?"))
  
  console.log(event)
  console.log(url)
  
  //Network, fall back to cache
  
  event.respondWith((async function(){
    let cache = await caches.open(maincache)
    let response;
    
    async function fromcache() {
      console.log("Cache")
      return caches.match(url)
    }
    
    async function fromnetwork() {
      return new Promise(function(resolve, reject){
        fetch(event.request).then(function(response){
          cache.put(url, response.clone())
          console.log("Network")
          resolve(response)
        })
        setTimeout(reject, 1600, "Request took too long")
      })
    }
    
    try {
      await fromnetwork()
    }
    catch (e){
      await fromcache()
    }    
    
    console.log(response)
    return response
  }()))
  

}



function handleinstall(event) {
  
  caches.open(maincache).then(function(cache){
    cache.keys().then(function(keys){
      console.log(keys)
    })
  })
  
  //Install ServiceWorker Immediently
  self.skipWaiting()
}

function handleactivate() {
  
  /*event.waitUntil(function )
  
  */
  
  //Take over open tabs
  clients.claim()
}

self.addEventListener("fetch", handlefetch) 
self.addEventListener("install", handleinstall)
self.addEventListener("activate", handleactivate)
