async function handlefetch(event) {
  let url = event.request.url
  console.log(event)
  console.log(url)
  
  console.log(event.request)
  event.request.url = url.slice(0,url.indexOf("?"))
  console.log(event.request)
  let response = fetch(event.request)
  
  
  event.respondWith(response)
}

self.addEventListener("fetch", handlefetch) 
