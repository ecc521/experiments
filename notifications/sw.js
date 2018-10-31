async function handlefetch(event) {
  console.log(event)
  reutrn fetch(event.request)
}

self.addEventListener("fetch", handlefetch) 
