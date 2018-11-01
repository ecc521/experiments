async function handlefetch(event) {
  let url = event.request.url
  
  console.log(event)
  console.log(url)
  
  
  
  let response = fetch(event.request)
  let cache = caches.open("rivers.run")
  cache.put(url, response)

  let keys = await cache.keys()
  console.log(keys)
  
  event.respondWith(response)
}

self.addEventListener("fetch", handlefetch) 
