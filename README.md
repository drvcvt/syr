# syr

**syr**
save your rust · strip your relics · single-yield-renamer · sick young refactorer

`syr` ist ein kleines CLI‑Tool, das Rust‑Dateien direkt im Dateisystem umschreibt. Es erkennt unbenutzte lokale Variablen und passt ihre Namen automatisch an.

## Features

* erkennt lokale Variablen, die deklariert, aber nie benutzt wurden
* unused locals werden nach `_name` umbenannt
* wenn eine zuvor geprefixte Variable (z. B. `_foo`) später benutzt wird, wird sie automatisch wieder zu `foo`
* der gesamte Code wird über `prettyplease` erneut formatiert
* renaming basiert auf „Basenamen“: `foo`, `_foo`, `__foo` gehören logisch zur gleichen Gruppe
* mehrfacher Durchlauf ist idempotent: die Datei bleibt stabil
* exit code 0 bei Erfolg, ≠0 bei Fehlern

## Wichtige Einschränkungen

* Kommentare werden **nicht** erhalten. `syn` entfernt sie beim Parsen komplett und `prettyplease` kann sie nicht rekonstruieren.
* Für einfache Identifier gedacht; komplexe Macro‑Kontexte werden nicht komplett unterstützt.

## Verwendung

syr arbeitet auf einer einzigen Datei:

syr pfad/zur/datei.rs

Ablauf:

1. Datei als AST parsen
2. Deklarationen und Nutzungen pro Basename sammeln
3. Für jede Variable bestimmen:

   * `_name`, wenn nie genutzt
   * `name`, wenn genutzt
4. Alle entsprechenden Bindungen und Expr‑Paths umschreiben
5. Datei formatiert zurückschreiben

## Installation

### Von Source lokal

1. Repository klonen
2. Ins Projekt wechseln
3. Installieren via Cargo

### Direkt über Cargo

cargo install --git [https://github.com/](https://github.com/)<username>/syr

`<username>` durch deinen GitHub‑Namen ersetzen.

## Lizenz

MIT License

Copyright (c) 2025 Matti

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

Der obige Copyright-Hinweis und dieser Genehmigungshinweis müssen in allen Kopien oder wesentlichen Teilen der Software enthalten sein.

DIE SOFTWARE WIRD OHNE JEDE AUSDRÜCKLICHE ODER IMPLIZITE GARANTIE BEREITGESTELLT, EINSCHLIESSLICH DER GARANTIE DER MARKTGÄNGIGKEIT, DER EIGNUNG FÜR EINEN BESTIMMTEN ZWECK UND DER NICHTVERLETZUNG. IN KEINEM FALL SIND DIE AUTOREN ODER COPYRIGHTINHABER FÜR ANSPRÜCHE, SCHÄDEN ODER ANDERE HAFTUNG VERANTWORTLICH, SEI ES AUS VERTRAG, UNERLAUBTER HANDLUNG ODER ANDERWEITIG, DIE AUS ODER IN VERBINDUNG MIT DER SOFTWARE ODER DER VERWENDUNG ODER SONSTIGEN GESCHÄFTEN MIT DER SOFTWARE ENTSTEHEN.

MIT‑Lizenz. Siehe LICENSE für Details.
