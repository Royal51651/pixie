<script>
    import { invoke } from "@tauri-apps/api/core";

    let file = $state(null);
    let imageUrl = $state("");
    let image = $state("");
    let input_image = "";
    let status = $state("Sort!");
    let settingsMode = $state(false);
    let red = $state("255");
    let green = $state("255");
    let blue = $state("255");

    const handleFileChange = (
        /** @type {{ target: { files: null[]; }; }} */ e,
    ) => {
        file = e.target.files[0];
        if (file) {
            imageUrl = URL.createObjectURL(file);

            const reader = new FileReader();
            reader.onload = () => {
                // @ts-ignore
                input_image = reader.result.split(",")[1];
            };

            reader.readAsDataURL(file);
        }
    };

    /**
     * @param {{ preventDefault: () => void; }} event
     */
    async function submit(event) {
        status = "Sorting...";
        event.preventDefault();
        image = await invoke("process", {
            input: input_image,
            red: Number(red),
            green: Number(green),
            blue: Number(blue),
        });

        // converts the image to a blob
        const bytes = atob(image);
        const byteArray1 = [];
        for (let i = 0; i < bytes.length; i++) {
            const byte = bytes.charCodeAt(i);
            byteArray1.push(byte);
        }
        const byteArray = new Uint8Array(byteArray1);
        const blob = new Blob([byteArray], { type: "image/png" });
        imageUrl = URL.createObjectURL(blob);
        status = "Image Sorted!";
        setTimeout(() => {
            status = "Sort!";
        }, 2000);
    }
    /*
    const changeBackgroundColor = () => {
        const color1 = "rgb(" + String(255 - parseInt(red)) + ", " + String(255 - parseInt(green)) + ", " + String(255 - parseInt(blue)) + ")";
        const color2 = "rgb(" + red + ", " + green + ", " + "blue" + ")";
        document.documentElement.style.background = "linear-gradient(135deg, " + color1 + ", " + color2 + ")";
    }
    */

    const toggleSettings = () => {
        settingsMode = !settingsMode;
        //changeBackgroundColor();
    };

    



</script>

<main class="container">
    {#if !settingsMode}
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
                <button class="buttons" onclick={toggleSettings}>Settings</button>
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
    {:else}
        <div class="settingsRow">

            <button class="buttons" onclick={toggleSettings}>Back</button>
        </div>
        <div class="settingsRow">
            <form style="width: 100%;">
                <input
                    class="colorSelect"
                    type="range"
                    bind:value={red}
                    min="0"
                    max="255"
                    style="accent-color: red;"
                />
                <input 
                    class="colorSelect" 
                    type="text" 
                    bind:value={red} 
                    style="color: red;"
                />
                <input
                    class="colorSelect"
                    type="range"
                    bind:value={green}
                    min="0"
                    max="255"
                    style="accent-color: green;"
                />
                <input 
                    class="colorSelect" 
                    type="text" 
                    bind:value={green}
                    style="color: green;"
                />
                <input
                    class="colorSelect"
                    type="range"
                    bind:value={blue}
                    min="0"
                    max="255"
                    style="accent-color: blue;"
                />
                <input 
                    class="colorSelect" 
                    type="text" 
                    bind:value={blue} 
                    style="color: blue;"
                />
                <h1 style="color: rgb({red}, {green}, {blue});">
                    Target Color
                </h1>
            </form>
        </div>
    {/if}
</main>

<style>
    .colorSelect {
        width: 100%;
        height: 5vh;
        padding: 0px;
        display: flex;
        color: #ffffff;
        border-radius: 8px;
        border: 1px solid transparent;
        justify-content: center;
        font-size: 3vh;
        font-weight: 500;
        font-family: inherit;
        align-items: center;
    }

    :root {
        font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
        font-size: 20px;
        line-height: 24px;
        font-weight: 400;

        color: #0f0f0f;
        font-synthesis: none;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        -webkit-text-size-adjust: 100%;
    }

    .settingsRow {
        padding: 0px;
        display: flex;
        height: 25%;
        width: 100%;
        max-height: 100%;
    }

    .imageDisplay {
        padding-left: 0px;
        padding-right: 0px;
        padding-bottom: 0px;
        padding-top: 1vh;
        margin: 0px;
        display: flex;
        justify-content: center;
        align-items: center;
        height: 85vh;
        width: 100vw;
        overflow: hidden;
        max-width: 100%;
        max-height: 100%;
    }

    .imageDisplay img {
        max-width: 100%;
        max-height: 100%;
        object-fit: contain;
    }

    img {
        padding: 0px;
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

    .buttonArea {
        padding-left: 0px;
        padding-right: 0px;
        width: 100%;
        display: flex;
        height: 5vh;
    }

    .buttons {
        width: 100%;
        height: 100%;
        font-size: 3vh;
        display: flex;
        height: auto;
        justify-content: center;
        align-items: center;
    }

    .submitArea {
        display: flex;
        padding-top: 1vh;
    }
    .submitArea input {
        border-radius: 8px;
        font-size: 3vh;
        display: flex;
        align-items: center;
    }
    input {
        width: 100vw;
        height: 100%;
    }

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
            background: linear-gradient(135deg,rgb(200, 9, 217),rgb(17, 135, 148));
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
