#  "UR10-Roboterarm" RFS-Projekt

## Entwicklung einer Ansteuerung und Benutzeroberfläche für den UR10, als Projekt im Fach RFS.

Im Rahmen des Faches RFS versuchen wir an dem Roboterarm ein Softwareprojekt umzusetzen. Das Thema des Projektes lautet "Schach".
Dazu soll der Roboter zunächst in der Lage sein gespeicherte Schachspiele demonstrativ nachzuspielen. Diese Aufgabe lässt sich zunächst in folgende Teilaufgaben unterteilen:

- Erste Ansteuerung des Roboters
- Lineare Bewegung zwischen zwei festen Koordinaten
- Dritte Achse zur Bewegung hinzufügen (Bogen nach oben zwischen zwei Punkten)
- Konzeption der benötigten Spielfiguren erstellen
- Mock-Up der GUI erstellen
- Figuren greifen können
- Figuren absetzen können
- Eingelesenes (Schach-)Dateiformat als Bewegungsanleitung verwenden

## Milestone 0

Die Projektplanung wurde so weit abgeschlossen. Softwaredesign wurde vom Team ausdiskutiert und beschlossen. Die Kommunikationsschnittstellen zwischen dem Front- und Backend wurden ermittelt. Aufgaben wurden ausformuliert und verteilt.  
Entwicklungsumgebung wurde eingerichtet und die Simulationsumgebung läuft auf den Maschinen des Teams.

## Milestone 1

Der Roboter sollte in der Lage sein Figuren zuverlässig von einem Feld zu einem anderen zu bewegen. Die Bewegungsabläufe sollten automatisch aus einer Datei entnommen und umgesetzt werden. Beide Spieler werden hierbei von dem Roboter simuliert. Auf der Benutzeroberfläche werden die Züge synchron dargestellt und der User hat grundlegende Funktionen wie "Start", "Stop" oder "Pause" zur Auswahl.

## Milestone 2

User übernehmen die Rollen der beiden Spieler. Ihre Eingaben werden über die GUI erfasst und durch den Roboter realisiert. 
Hierzu muss die GUI interaktiver gestaltet werden - ein Spielablauf muss live aufgezeichnet und dem Roboter übergeben werden.

## Milestone 3

Einer der Spieler wird nun vom Roboter übernommen. Nachdem der User seinen Zug über die GUI gemacht hat reagiert der Roboter mit seinem eigenen Spielzug.
Damit dies geschehen kann, muss eine Spiellogik implementiert werden. Je nach genauer Umsetzung wird zusätzlich eine Fehlerüberprüfung benötigt. Andernfalls könnte der User illegale Züge ausführen. 

![grafik](https://github.com/adrianmoerk/RoboterArmProjekt/assets/128092143/bbd14dcc-f293-4e71-95c0-c09623bd43ec)


## Features/Bugs

Damit später eine gute Projektdokumentation (und zwischendrin gute Protokolle) erstellt werden kann bitte regelmäßig und gewissenhaft Issues anlegen und mit den entsprechenden Labels versehen. Folgende Labels wurden dazu zusätzlich erstellt:

- Information: Dinge die zur Kenntnis genommen werden sollten
- Frontend: Die Gruppe der Benutzeroberfläche betreffende Issues
- Backend: Die Gruppe der Ansteurerung betreffende Issues

Sollten weitere Labels benötigt werden, können diese einfach erstellt werden. Bitte entsprechende Beschreibung hinzufügen um die Verwendung zu erleichtern.
