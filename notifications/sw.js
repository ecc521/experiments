const maincache = "rivers.run"

function handlefetch(event) {
  let url = event.request.url
  
  //USGS data is date dependent. We should therefore ignore the query parameter
  url = url.slice(0, url.indexOf("?"))
  
  console.log(event)
  console.log(url)
  
  //Network, fall back to cache
  let response, cache;
  
  response = fetch(event.request)
  cache = await caches.open(maincache)
  

  response.then(function(response){
    cache.put(url, response.clone())
    return response
  })
  response.catch(function(){
      return cache.match(url)
  })
  
  
  event.respondWith(response)
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
