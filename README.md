# 🪐 SolBalance: Delta-Neutral Yield Harvester

![Solana](https://img.shields.io/badge/Solana-362D59?style=for-the-badge&logo=solana&logoColor=white)
![Anchor](https://img.shields.io/badge/Anchor-Rust-E34234?style=for-the-badge&logo=rust&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)

**SolBalance** to autonomiczny, pozbawiony powiernictwa (non-custodial) agregator zysków zbudowany na Solanie. Posiada zintegrowany algorytm on-chain (Anchor Smart Contract) oraz off-chainowy mózg roboczy (The Scout), który zapewnia Delta-Neutralną stymulację kapitału.

---

## 🔄 Cykl życia transakcji w Agregatorze

### Krok 1: Wpłata i Analiza (The Scout)
Użytkownik wpłaca kapitał do protokołu (np. 1000 USDC).
Nasz algorytm pracujący w tle (**Scout**) nieustannie skanuje rynek Solany w poszukiwaniu "Skew" (przechyleń rynkowych). Przykładowo:
- Jupiter: Funding = 25% APR
- Drift: Funding = **45% APR** (Ekstremalnie dużo zagrań na wzrost/Longów).
- Zeta: Funding = 30% APR.
**Decyzja Algorytmu:** Wybieramy Drift. "Ulica" pcha cenę w górę, bardzo potrzebuje kogoś kto otworzy przeciwny zakład (Shorta), dlatego płaci gigantyczny yield.

### Krok 2: Egzekucja Delta-Neutral (The Split)
Agregator błyskawicznie dzieli te środki na pół, budując idealnie zrównoważoną wagę (Delta = 0).
- **Noga "LONG" (Spot):** Za 500 USDC kupuje "fizyczną" Solanę uderzając w Jupitera. Zamiast zwykłego SOL używamy LST Caching'u – kupujemy `jitoSOL`, co automatycznie daje zyski ze stakingu (~8%).
- **Noga "SHORT" (Perp):** Za drugie 500 USDC otwiera bezpieczną pozycję Short na protokole Drift (dźwignia 1x).

> **Dlaczego to odporne na krew na rynku (Delta-Neutral)?**
> Jeśli cena SOL wzrośnie o 10%, noga Long na `jitoSOL` zyskuje 50$. Niestety noga Short na Perpie traci 50$. 
> Twój wynik cenowy to równe **0$**. Wyeliminowaliśmy wpływ ruchów rynkowych na wycenę kryptowaluty.

### Krok 3: Zbieranie "Nagrody" (The Harvesting)
Dzięki silnemu zapotrzebowaniu na stabilność sieci na Drifcie (ogrom Longów), co godzinę/epokę nasz serwer inkasuje potężną marżę (Funding Payment) i oddaje ją Shortującym. Inteligentny Kontrakt przeprowadza **Auto-Compounding** w tle – zyski dodawane są do ogólnego balansu jako nagroda.

### Krok 4: Rebalancing (The Brain)
Rynki to żywy organizm. Nagle Funding na Drifcie opada do 10%, z kolei na protokole Mango wybucha nowa szał-hossa windując stopę do 80% APR. Nasz worker (**Brain**) wykrywa to w 10 sekund. W trybie natychmiastowym zamyka pozycje (Drift) i przenosi obroty za pomocą CPI na drugą stronę płotu (Mango), dbając o wyciskanie z rynku najwyższych zysków w każdej mikrosekundzie bez poślizgu cenowego.

---

## 📊 Scenariusze Wygranej Rynkowej (Tabela Payoutów)

Z SolBalance omijasz ryzyko kierunku giełdy. Interesuje Cię tylko zmienność stopy popytu na kontrakty Futures.

| Czego spodziewa się "Ulica"? | Wynik na jitoSOL (Long) | Wynik na Drifcie (Short) | Zysk Agregatora |
| :--- | :--- | :--- | :--- |
| **Hossa / Bull Run (+20%)** | +100$ | -100$ | **+ Funding** (Gigantyczny! Longi płacą nagrodę) |
| **Bessa / Bear Market (-20%)** | -100$ | +100$ | **+ Funding** (Bardzo Niski / Zerowy) |
| **Boczniak / Consolidation (0%)**| 0$ | 0$ | **+ Funding** (Stabilny dochód pasywny) |

### 🚨 Krytyczne Punkty Zabezpieczeń w Kodzie
Agregator musi być przygotowany na drastyczne obniżenie optymizmu rynków (głęboką bessę), kiedy ludzie rzucają się na shortowanie Solany ratując kapitał. Wówczas protokół Drift **wymaga zapłaty za utrzymywanie pozycji Short** (Ujemny Funding Rate).

Nasz program posiada wbudowaną mechanikę on-chain ucieczki (*Panic Button - Emergency Evacuation*):
Kiedy zwiadowca off-chain (Scout) dotknie Funding Rate poniżej 0, **Agregator ewakuuje całe środki na powrót poprzez swapy Jupitera**, rozwiązując ubezpieczenia kontraktowe zpowrotem do chłodnego, bezpiecznego zasobu czystego USDC. Zyski i straty zostają zablokowane unikając kosztów.
