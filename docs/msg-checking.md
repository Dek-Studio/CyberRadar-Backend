
# Message checking and keywords dictionary

## 1. Purpose and objectives

This module provides a mechanism to analyze text inputs (such as emails and text messages) for indicators of data phishing, financial fraud, and social engineering manipulation.

## 2. Operating principles

To determine the level of threat, the algorithm evaluates text using three core components:

* **[Weighted Keyword Dictionary](../keywords.ron)** tailored for common threat indicators.
* **URL Detection** - built-in pattern matching to identify embedded web links.
* **Phrase Synergies:** Contextual analysis tracking how flagged keyword categories interact and amplify risk within the same message.

These components combine to calculate a cumulative risk score representing the likelihood that a given message is malicious.

### Total score calculation

$
    S = S_{kwd} + P_{url} + P_{syn}
$

**Where:**  
    $S$ - Total risk score assigned to the message.  
    $S_{kwd}$ - Sum of weights of all flagged keywords found in the message.  
    $P_{url}$ - Flat bonus of **3 points** added if the message contains a URL.  
    $P_{syn}$ - Bonus points added for detected social engineering keyword synergies/combinations.

### Synergy detection ($P_{syn}$)

To detect sophisticated social engineering attacks, the algorithm evaluates the set of matched keyword categories present in a message, checking for predefined high-risk combinations or intersections between specific categories and the presence of a URL.

#### Defined combinations

| Found categories | Bonus points | Justification |
| :--- | :--- | :--- |
| presja_czasu_naganianie + bankowosc_finanse | 3 | Pressuring the target for immediate action under threat of losing funds or account suspension. |
| presja_czasu_naganianie + doplata_paczka | 2 | Threatening to withhold or return a parcel to the warehouse unless immediate payment is made. |
| podstepne_wiado_znajomi + URL | 3 | Impersonation scam (e.g., "son/daughter in trouble"), designed to trick the target into urgent money transfers. |

### Score-to-Probability Mapping

| Score | Threat propability |
| :--- | :--- |
| 0 | 0% |
| 1 - 2 | 25% |
| 3 - 4  | 50% |
| 5 - 6 | 75% |
| 7 + | 100% |

## 3. Dictionary structure

The keyword dictionary is defined as a collection of `Keyword` structs conforming to the following structure (written in Rust/RON syntax):

```rs
struct Keyword {
    /// The keyword or phrase itself (e.g., "natychmiast").
    value: String,
    /// Risk weight assigned to the keyword, ranging from 1 to 3.
    weight: u32,
    /// Category identifier for synergy tracking.
    category: String,
}
```

### Available categories:

* `presja_czasu_naganianie` (Time pressure / Urgency)
* `bankowosc_finanse` (Banking & Finance)
* `podejrzane_linki_skroty` (Suspicious links & URL shorteners)
* `doplata_paczka` (Parcel delivery fees / Surcharges)
* `podstepne_wiado_znajomi` (Deceptive friend/family messages)
* `energetyka_rachunki` (Utilities & Bills)
* `naganianie_okazje_wygrane` (Promotions, sweepstakes & false wins)