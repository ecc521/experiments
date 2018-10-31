async function handlefetch(event) {
  console.log(event)
  return fetch(event.request)
}

self.addEventListener("fetch", handlefetch) 
