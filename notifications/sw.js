function handlefetch(event) {
  let url = event.request.url
  
  //USGS data is date dependent. We should therefore ignore the query parameter
  url = url.slice(0, url.indexOf("?"))
  
  console.log(event)
  console.log(url)
  
  //Network, fall back to cache
  let response, cache;
  
  response = fetch(event.request)
  
  response.then(function(response){
    cache.put(url, response)
  })
  
  response.catch(function(event){
      caches.open("rivers.run").then(function(cache){
          response = cache.match(url)
      })
  })
  
  event.respondWith(response)
}

function handleinstall(event) {
  cache.keys().then(function(keys){
    console.log(keys)
  })
  
  //Set this as the active serviceworker
  self.skipWaiting()
  //Take over open tabs
  clients.claim()
}



self.addEventListener("fetch", handlefetch) 
self.addEventListener("install", handleinstall)
