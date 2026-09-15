# CyberRadar-Backend

Backend API server for **[CyberRadar](https://github.com/Dek-Studio/CyberRadar-Web)**. Written in **Rust**.

## Function

The point of this backend is calling **[CERT.PL](https://cert.pl)** advisories API, filtering the response to get only information usefull to our app and expose it.
In future possibly also exposing other data needed by app.

## Available endpoints

* /advisories.json