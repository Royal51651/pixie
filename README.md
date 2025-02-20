# What is Pixie?

Pixie is a blazingly fast app that sorts the pixels in images based on their color! Pixie was made as an exercise to learn the Svelte framework, and get more familiar with Rust.

# How do I use it!

Pixie is shockingly simple. You input an image, and press sort! In seconds, the image will get sorted! From there, just right click the image, and you should be able to save to your device!

# How does it work?

Pixie sorts images based on their deviation from a target color. For example, if you wanted to sort pixels by brightness, you could pass in pure white. The target color is completely up to you, and can be configured in the settings menu. The background will even change to reflect the image

# When would this ever be useful?

We don't talk about how useful this is...

# How do download it?

You should be able to find the latest release by navigating to the release tab here:
https://github.com/Royal51651/pixie/releases

However, you can also compile from source if you like:

Make sure you have git, and follow the prerequisites from the official Tauri site: https://v2.tauri.app/start/prerequisites/

Once you're done with that, clone the directory.

```git clone https://github.com/Royal51651/pixie```

Then cd into the folder

```cd SimpleCalc```

Install the dependencies

```npm install```

And finally run the project

```npm run tauri build --release```

From there, you should be presented with instructions to install based on your operating system. Happy sorting!
