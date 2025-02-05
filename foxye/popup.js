document.getElementById("save").addEventListener("click", async () => {
    let [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
    chrome.scripting.executeScript({
      target: { tabId: tab.id },
      function: sendPageContent
    });
  });
  
  function sendPageContent() {
    let content = document.body.innerText;
    let url = window.location.href;
  
    fetch("https://your-api-endpoint.com/save", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ url, content })
    });
  }
  