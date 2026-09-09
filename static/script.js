console.log("script.js success loading!");

const form = document.getElementById("urlForm");

form.addEventListener("submit", async (event) => {
    event.preventDefault();
    console.log("submit caught by JavaScript!");

    const input = document.getElementById("inputUser");

    const data = new URLSearchParams();
    data.append("url", input.value);


    const response = await fetch("/post_url", {
        method: "POST",
        body: data
    });

    const result = await response.json();
    console.log("API response:", result);

    console.log(result);

    const resultElement = document.getElementById("result");

    console.log("HTML element:", resultElement);
    console.log("Short code:", result.short_code);

    resultElement.textContent = result.short_code;

    const shortUrl = `${window.location.origin}/url/${result.short_code}`;
    resultElement.href = shortUrl;
    resultElement.textContent = shortUrl;
});

