async function handlefetch(event) {
  let url = event.request.url
  
  //USGS data is date dependent. We should therefore ignore the query parameter
  url = url.slice(0, url.indexOf("?"))
  
  console.log(event)
  console.log(url)
  
  //Network, fall back to cache
  let response, cache;
  
  response = await fetch(event.request)
  cache = await caches.open("rivers.run")

  response.catch(function(event){
      response = cache.match(url)
  })

  cache.put(url, response)

  let keys = await cache.keys()
  console.log(keys)
  
  
  event.respondWith(response)
}

self.addEventListener("fetch", handlefetch) 
