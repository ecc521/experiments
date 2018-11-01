async function handlefetch(event) {
  let url = event.request.url
  
  //USGS data is date dependent. We should therefore ignore the query parameter
  url = url.slice(0, url.indexOf("?"))
  
  console.log(event)
  console.log(url)
  
  //Network, fall back to cache
  let response, cache;
  
  response = fetch(event.request)
  
  response.then(function(){
    cache.put(url, response)
  })
  
  response.catch(function(event){
      caches.open("rivers.run").then(function(cache){
          response = cache.match(url)
      })
  })


  let keys = await cache.keys()
  console.log(keys)
  
  
  event.respondWith(response)
}

self.addEventListener("fetch", handlefetch) 
