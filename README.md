<p align="center">
    <img src="icon.ico" alt="icon" width="200"/>
</p>
<h1 align="center">
    Philia
</h1>

<p align="center">
    <img alt="GitHub all releases" src="https://img.shields.io/github/downloads/MaximumOverflow/Philia/total?style=for-the-badge">
    <img alt="GitHub release (latest by date including pre-releases)" src="https://img.shields.io/github/v/release/MaximumOverflow/Philia?include_prereleases&style=for-the-badge">
    <img alt="GitHub issues" src="https://img.shields.io/github/issues/MaximumOverflow/Philia?color=%23366ace&style=for-the-badge">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/MaximumOverflow/Philia?color=%23dfb317&style=for-the-badge">
</p>

Philia is a simple imageboard scraping application with extensive support for AI dataset creation.

## Features

### Search
[<img src="images/search.gif" align="right" width="512"/>](images/search.gif)
- Tag auto-completion in the search field.
- Support for many of the most widely used imageboards
- Easily add support for your own imageboards through scripting.

<p>&nbsp;</p>
<p>&nbsp;</p>
<p>&nbsp;</p>
<p>&nbsp;</p>
<p>&nbsp;</p>
<p>&nbsp;</p>
<p>&nbsp;</p>

### Download
[<img src="images/download.gif" align="right" width="512"/>](images/download.gif)
- Download hundreds of images at once.
- Or select which images to download individually.
- Quickly add your downloaded images to any existing dataset.

<p>&nbsp;</p>
<p>&nbsp;</p>
<p>&nbsp;</p>
<p>&nbsp;</p>
<p>&nbsp;</p>

### Personalize
[<img src="images/personalize.gif" align="right" width="512"/>](images/personalize.gif)
- Edit the image grid layout to fit your needs
- Modify the accent color to your liking.
- Support for light mode and dark mode.

<p>&nbsp;</p>
<p>&nbsp;</p>
<p>&nbsp;</p>
<p>&nbsp;</p>
<p>&nbsp;</p>

### Manage your datasets
[<img src="images/export.gif" align="right" width="512"/>](images/export.gif)
- Easily create datasets from the images you download.
- Filter tags and tag categories to remove the problematic ones.
- Escape tag parentheses.
- Replace tag underscores with spaces.
- Resize your images.
- Apply letterboxing.
- Convert your images to several different formats.
- Export your datasets for LoRA and DreamBooth training.

## Default sources
Additional sources can be added by creating a simple [Rhai](https://rhai.rs/) script and adding it to the *sources* folder.  
Take a look at the available scripts for reference.

- Danbooru
- Gelbooru
- Safebooru
- E926
- E621
- Rule34
