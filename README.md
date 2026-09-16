![](repo_cover.png)

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
            "title": "📦 „Problem z dostawą” może być próbą wyłudzenia danych",
            "slug": "problem-z-dostawa-moze-byc-proba-wyludzenia-danych",
            "lead": "🔎 CERT Polska obserwuje kampanię phishingową, w której przestępcy podszywają się pod firmę kurierską UPS.",
            "description": "<p>W wiadomości e-mail informują o rzekomej nieścisłości w danych dotyczących dostawy i kierują odbiorcę na stronę, która ma pomóc w rozwiązaniu problemu. W rzeczywistości formularz przygotowany przez przestępców służy do pozyskania danych osobowych i danych kart płatniczych 💳</p><p>🔐 Warto sprawdzić, czy wiadomość rzeczywiście pochodzi od firmy kurierskiej, a informacje o przesyłce zweryfikować niezależnie, np. za pośrednictwem oficjalnego serwisu przewoźnika. Dobrą praktyką jest również uważne sprawdzanie adresu strony oraz nadawcy przed podaniem jakichkolwiek danych 🌐</p><p>📩 Podejrzane wiadomości e-mail i strony zgłaszaj do CERT Polska przez <a href=\"http://incydent.cert.pl\">incydent.cert.pl</a> lub w aplikacji mObywatel, wusłudze „Bezpiecznie w sieci”. Twoje zgłoszenie pomaga nam chronić Ciebie i innych użytkowników!</p>",
            "publication_date": "2026-09-16T17:30:00.012583+02:00"
        },
        {
            "title": "„Witamy w Amazon Prime” - uważaj na fałszywe wiadomości o subskrypcji",
            "slug": "witamy-w-amazon-prime-uwazaj-na-falszywe-wiadomosci-o-subskrypcji",
            "lead": "🔎 CERT Polska obserwuje kolejną falę kampanii phishingowej wykorzystującej wizerunek firmy Amazon.",
            "description": "<p>Cyberprzestępcy rozsyłają e-maile informujące o rzekomym uruchomieniu subskrypcji - niżej znajduje się informacja o możliwości anulowania usługi. Link prowadzi do serwisu przygotowanego przez oszustów, gdzie użytkownik proszony jest o podanie hasła do konta powiązanego z jego adresem e-mail oraz danych karty płatniczej. Wprowadzone informacje trafiają bezpośrednio do przestępców 💳</p><p>🧠 To dobrze znany mechanizm socjotechniczny. Nadawcy próbują wywołać poczucie zagrożenia związane z niechcianą opłatą, jednocześnie podsuwając pozornie proste rozwiązanie.</p><p>🔐 Jak chronić swoje dane?</p><p>🔹 Nie działaj pod wpływem presji. Informacja o nieoczekiwanym wydatku ma skłonić cię do nieprzemyślanego działania.</p><p>🔹 Sprawdzaj uważnie adres nadawcy oraz adres strony internetowej.</p><p>🔹 W razie wątpliwości status subskrypcji zweryfikuj samodzielnie na oficjalnej stronie firmy.</p><p>🚨 Podejrzane wiadomości oraz strony internetowe zgłaszaj do CERT Polska przez <a href=\"http://incydent.cert.pl\">incydent.cert.pl</a> lub w aplikacji mObywatel, korzystając z usługi „Bezpiecznie w sieci”.</p>",
            "publication_date": "2026-09-15T18:00:00.036062+02:00"
        },
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