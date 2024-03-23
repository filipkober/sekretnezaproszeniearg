# entr0py

## 🇵🇱
entr0py to coś w stylu gry-łamigłówki. Jej celem było ukończenie 18 poziomów, każdy z których wymagał odgadnięcia kodu w postaci ciągu cyfr. To mały projekt, którego celem było ujawnienie zaproszenia na moje urodziny po ukończeniu wszystkich poziomów. Projekt składa się z dwóch części: [aplikacji konsolowej (tego)](https://github.com/filipkober/sekretnezaproszeniearg) i [strony internetowej](https://github.com/filipkober/entr0py), na której zawarte były niektóre łamigłówki, oraz panel administracyjny wraz z podglądem postępu uczestników, dostępnym dla tych którzy ukończyli wszystkie poziomy. Aplikacja chodziła na chmurze, na darmowym serwerze Oracle, do przypisania domeny użyłem [nginx](https://www.nginx.com/) i [certbot](https://certbot.eff.org/) do HTTPS.

### technologie
aplikacja napisana jest w [Rust](https://www.rust-lang.org/), przy użyciu biblioteki [tui](https://docs.rs/tui/latest/tui/). Zapytania wykonywane są przy użyciu biblioteki [reqwest](https://docs.rs/reqwest/latest/reqwest/)

## 🇺🇸
entr0py is something like a puzzle game. Its goal was to complete 18 levels, each requiring a code made from digits. It's a small project with the goal of revealing a birthday party invitation after completing every level. This project consists of two parts: [a console app](https://github.com/filipkober/sekretnezaproszeniearg) and [a website (this)](https://github.com/filipkober/entr0py), where some of the puzzles were located, along with an admin panel from which you could see others' progress. Panel access was available after completing all levels. I hosted the website on a free web server provided by Oracle, to assign a domain I used [nginx](https://www.nginx.com/) and [certbot](https://certbot.eff.org/) for HTTPS.

### technologies
the application is written in [Rust](https://www.rust-lang.org/), using a library called [tui](https://docs.rs/tui/latest/tui/). To make web requests I used [reqwest](https://docs.rs/reqwest/latest/reqwest/)
