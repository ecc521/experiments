async function handlefetch(event) {
  let url = event.request.url
  
  console.log(event)
  console.log(url)
  
  //Network, fall back to cache
  let response, cache;
  response = fetch(event.request)
  
  cache = await caches.open("rivers.run")

  response.catch(function(event){
      //USGS data is date dependent. We should therefore ignore the query parameter
      response = cache.match(request, {ignoreSearch:true})
  })

  cache.put(url, response)

  let keys = await cache.keys()
  console.log(keys)
  
  
  event.respondWith(response)
}

self.addEventListener("fetch", handlefetch) 
