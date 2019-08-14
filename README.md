
# OSIMINT_alert

<p align="center">
        <a> <img src="https://img.shields.io/github/last-commit/audeberc/static-maps-monitor?style=flat-square" /></a>
        <a> <img src="https://img.shields.io/github/license/audeberc/static-maps-monitor" /></a>
        <a> <img src="https://travis-ci.com/audeberc/OSIMINT-alert.svg?branch=master" /></a>
</p>

A simple Rust-based tool to monitor data updates in common static-map services (Google, Yandex, Bing, Wikimapia ...) over user-defined areas. 

## Motivation 

Large scale Open Source Image Intelligence [(OSIMINT)](https://en.wikipedia.org/wiki/Open-source_intelligence) requires automatic monitoring of open data sources in order to be alerted when new imagery or Open source Geodata has been added on an area of interest. 

This tool allows to set up recurring monitoring tasks of public mapping services on user-defined bounding boxes. When an data update on an area is detected, an alert is logged and a thumbnail of the area is saved. 

The tool work with either image data (Aerial & satellite imagery, raster maps) or Json (Wikimapia)

## Screenshot 

![loging_example](https://github.com/audeberc/static-maps-monitor/blob/master/ressources_readme/log.png)
![Illu](https://github.com/audeberc/OSIMINT-alert/blob/master/ressources_readme/illu_1.png)


## Build 

You need to install Rust in order to compile. Get started in a few seconds at https://www.rust-lang.org/learn/get-started

> cargo build --release

## Usage 
The tool takes a Json configuration file as single argument 

> ./OSIMINT_alert confs/exemple.json 

In the json you can specify
 * The name of your Area Of Interest
 * The imagery source ("Yandex", "Google", "Bing", "Wikimapia")
 * Your API key (for Google, Bing & Wikimapia)
 * The layer of interest
 
    * Yandex: "sat", "map", "trf" 
    * Google: "satellite", "roadmap", "hybrid", "terrain"
    * Bing: "Aerial", "AerialWithLabels", "Road"
    * Wikimapia: the category code as defined [here](https://gist.github.com/effrenus/6989940)
    
 * The bounding box coordinates 
 * The frequency at which to check for new imagery (You may want to limit your API usage) 

#### Remarks: 
   No matter the size of the provided bounding box, the fetched tiles are fixed size (quite small depending on the provider). 
   If new imagery is added to a very small part of the AOI (<1 pixel at the zoom level), the update might not be detected.
   
## Dependencies 
  TODO

## Features to be implemented soon:
  * Mail alerts
