<script>
  import { invoke } from "@tauri-apps/api/core";

  let file = $state(null);
  let imageUrl = $state("");
  let image = $state("");
  let input_image = "";
  let status = $state("Sort!");
  const handleFileChange = (/** @type {{ target: { files: null[]; }; }} */ e) =>{
    file = e.target.files[0];
    if(file) {
      imageUrl = URL.createObjectURL(file);
      
      const reader = new FileReader();
      reader.onload = () => {
        // @ts-ignore
        input_image = reader.result.split(',')[1];
      }

      reader.readAsDataURL(file);
    }
  }

  const reset = () => {
    file = null;
    imageUrl = "";
  }

  async function submit(event) {
    status = "Sorting..."
    event.preventDefault();
    image = await invoke("process", { input: input_image });
    console.log(image);

    // converts the image to a blob
    const bytes = atob(image);
    const byteArray1 = [];
    for (let i = 0; i < bytes.length; i++){
      const byte = bytes.charCodeAt(i);
      byteArray1.push(byte);
    }
    const byteArray = new Uint8Array(byteArray1);
    const blob = new Blob([byteArray], { type: 'image/png' });
    imageUrl = URL.createObjectURL(blob);
    status = "Image Sorted!";
    setTimeout(() => {
      status = "Sort!";
    }, 2000);
  }

</script>

<main class="container">

  {#if !imageUrl}
  <div class="row">   
    <h1>Pixie 1</h1>
  </div>
  <div class="row">   
    <h2>Select A File To Get Started</h2>
  </div>
  {/if}
  {#if imageUrl}
  <div class="buttonArea">
    <button class="buttons" onclick={submit}>{status}</button>
  </div>
  {/if}
  <div class="submitArea">
    <input type="file" onchange={handleFileChange} accept="image/*" />
  </div>
  <div class="imageDisplay">
    {#if imageUrl}
      <img src={imageUrl} alt="Sorter" />
    {/if}
  </div>

</main>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 20px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.imageDisplay {
  padding-top: 10px;
  height: 100%;
  width: 100%;
}

img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.row {
  display: flex;
  justify-content: center;
}

h1 {
  text-align: center;
}

.buttonArea{
  display: flex;
  justify-content: center;
}

.buttons {
  width: 100%;
}

.submitArea {
  display: flex;
  padding-top: 10px;
}

input {
  width: 100%;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

input {
  padding: 1vh;
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    --background-color: #997f3e;
    background: linear-gradient(135deg, rgb(164, 10, 225), rgb(17, 135, 148));
    background-size: 100%;
    background-attachment: fixed;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
