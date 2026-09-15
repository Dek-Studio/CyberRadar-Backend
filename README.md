# CyberRadar Backend

Backend API server for **[CyberRadar](https://github.com/Dek-Studio/CyberRadar-Web)**. Written in **Rust**.

## Function

The point of this backend is calling the **[CERT.PL](https://cert.pl)** advisories API, filtering the response to get only information useful to our app, and exposing it. In the future, it may also expose other data needed by the app.

## Available endpoints

* `/advisories.json` - Exposes a filtered and cleaned list of advisories from the 3 latest pages of the CERT API in JSON format. The response contains only advisories tagged as *"Dla użytkowników"*.

    **Example response:**
    ```json
    [
        {
            "title": "„Witamy w Amazon Prime” - uważaj na fałszywe wiadomości o subskrypcji",
            "slug": "witamy-w-amazon-prime-uwazaj-na-falszywe-wiadomosci-o-subskrypcji",
            "lead": "🔎 CERT Polska obserwuje kolejną falę kampanii phishingowej wykorzystującej wizerunek firmy Amazon.",
            "description": "<p>Cyberprzestępcy rozsyłają e-maile informujące o rzekomym uruchomieniu subskrypcji - niżej znajduje się informacja o możliwości anulowania usługi. Link prowadzi do serwisu przygotowanego przez oszustów, gdzie użytkownik proszony jest o podanie hasła do konta powiązanego z jego adresem e-mail oraz danych karty płatniczej. Wprowadzone informacje trafiają bezpośrednio do przestępców 💳</p><p>🧠 To dobrze znany mechanizm socjotechniczny. Nadawcy próbują wywołać poczucie zagrożenia związane z niechcianą opłatą, jednocześnie podsuwając pozornie proste rozwiązanie.</p><p>🔐 Jak chronić swoje dane?</p><p>🔹 Nie działaj pod wpływem presji. Informacja o nieoczekiwanym wydatku ma skłonić cię do nieprzemyślanego działania.</p><p>🔹 Sprawdzaj uważnie adres nadawcy oraz adres strony internetowej.</p><p>🔹 W razie wątpliwości status subskrypcji zweryfikuj samodzielnie na oficjalnej stronie firmy.</p><p>🚨 Podejrzane wiadomości oraz strony internetowe zgłaszaj do CERT Polska przez <a href=\"[http://incydent.cert.pl](http://incydent.cert.pl)\">incydent.cert.pl</a> lub w aplikacji mObywatel, korzystając z usługi „Bezpiecznie w sieci”.</p>"
        },
        {
            "title": "📱 Przestępcy podszywają się pod Play",
            "slug": "przestepcy-podszywaja-sie-pod-play",
            "lead": "📧 CERT Polska monitoruje kampanię phishingową, w której przestępcy podszywają się pod operatora sieci komórkowej.",
            "description": "<p>E-mail informuje o rzekomej konieczności opłacenia faktury i może wyglądać jak standardowa korespondencja od dostawcy usług.</p><p>🔗 W wiadomości znajduje się odnośnik prowadzący do formularza płatności. W rzeczywistości jest to fałszywa strona przygotowana przez oszustów. Kolejny etap ma skłonić odbiorcę do podania danych dostępu oraz danych karty płatniczej.</p><p>🛡️ Zachowaj czujność!</p><p>🔎 Sprawdzaj uważnie adres nadawcy oraz domenę strony, na której ma odbyć się płatność. Informację o rachunkach warto zweryfikować bezpośrednio w oficjalnym serwisie operatora, zamiast korzystać z odnośnika zawartego w wiadomości.</p><p>📩 Podejrzane wiadomości e-mail i strony internetowe zgłaszaj do CERT Polska w serwisie <a href=\"[http://incydent.cert.pl](http://incydent.cert.pl)\">incydent.cert.pl</a> lub w aplikacji mObywatel, w usłudze „Bezpiecznie w sieci”. </p><p></p>"
        }
    ]
    ```

## Configuration

The server is configured using environment variables:
* `PORT` - The port the server will listen on (default: `3000`).

## Setting up the server

This server is designed to run via **systemctl**. You can create the service file manually or use our pre-made template by following these steps:

1. Download the latest binary from [releases](https://github.com/Dek-Studio/CyberRadar-Web/releases) or compile it on your own. 
2. Download our pre-made service file: [cyberradar.service](/cyberradar.service).
3. Open the `cyberradar.service` file with any text editor and fill in the blanks (`PORT` and `EXECUTABLE PATH`) as indicated by the comments.
4. Place the edited service file in `/etc/systemd/system/` so the path becomes: `/etc/systemd/system/cyberradar.service`.
5. Enable and start the service by executing these commands:
    ```bash
    # Reload systemd to recognize the new service file
    sudo systemctl daemon-reload

    # Enable the service to start automatically on system boot
    sudo systemctl enable cyberradar

    # Start the service right now
    sudo systemctl start cyberradar
    ```

### How to monitor the service:

* Checking if it is running successfully:
    ```bash
    sudo systemctl status cyberradar
    ```
* Viewing logs:
    ```bash
    sudo journalctl -u cyberradar -f
    ```

# LICENSE

This project is distributed under the **CC BY-NC 4.0** license. For more information, check the [LICENSE](LICENSE) file.