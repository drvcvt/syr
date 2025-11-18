# syr

**s**ave **y**our **r**ust · **s**trip **y**our **r**elics · **s**ingle-**y**ield-**r**enamer · **s**ick **y**oung **r**efactorer

Ein CLI-Tool, das automatisch Unterstrich-Präfixe für unbenutzte Variablen in Rust-Dateien verwaltet.

> 📖 [English Version](README.md)

## Was es macht

`syr` analysiert deinen Rust-Code und:
- Präfixiert unbenutzte lokale Variablen mit `_` (z.B. `foo` → `_foo`)
- Entfernt das `_`-Präfix, wenn eine Variable benutzt wird (z.B. `_bar` → `bar`)
- Formatiert deinen Code mit `prettyplease`

## Schnellstart

### Installation

#### Von Source
```bash
git clone https://github.com/drvcvt/syr
cd syr
cargo install --path .
```

#### Über GitHub
```bash
cargo install --git https://github.com/drvcvt/syr
```

### Verwendung

Bearbeite eine einzelne Rust-Datei:
```bash
syr pfad/zur/datei.rs
```

Das Tool:
1. Parst die Datei als AST
2. Analysiert Variablen-Deklarationen und -Nutzungen
3. Benennt Variablen je nach Nutzung um:
   - `_name` wenn nie benutzt
   - `name` wenn benutzt
4. Schreibt alle Bindings und Expression-Paths um
5. Formatiert und speichert die Datei

## Features

- **Intelligentes Umbenennen**: Variablen mit dem gleichen Basisnamen (`foo`, `_foo`, `__foo`) werden als eine logische Gruppe behandelt
- **Idempotent**: Mehrfaches Ausführen produziert stabile Ergebnisse
- **Einfach**: Arbeitet mit einer Datei zur Zeit
- **Exit-Codes**: Gibt 0 bei Erfolg zurück, ungleich 0 bei Fehlern

## Wichtige Einschränkungen

- **Kommentare werden nicht erhalten**: `syn` entfernt Kommentare beim Parsen, und `prettyplease` kann sie nicht wiederherstellen
- **Nur einfache Identifier**: Komplexe Makro-Kontexte werden nicht vollständig unterstützt

## Wie es funktioniert

Das Umbenennen basiert auf "Basisnamen" - alle Varianten einer Variable (`foo`, `_foo`, `__foo`) werden logisch gruppiert. Das Tool entscheidet dann die richtige Form basierend auf der Nutzung:

- Variable deklariert aber nie benutzt → Präfix mit `_`
- Variable wird benutzt → `_`-Präfix entfernen
- Bereits korrekt präfixierte Variablen bleiben unverändert

## Lizenz

MIT License - siehe [LICENSE](LICENSE) für Details

Copyright (c) 2025 Matti
