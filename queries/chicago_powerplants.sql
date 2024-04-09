INSTALL spatial;
LOAD spatial;
CREATE TABLE powerplants AS SELECT * FROM ST_Read('datasets/powerplants.json');
CREATE TABLE chicago_bgs AS SELECT * FROM ST_Read('datasets/acs-chicago-bgs.geojson');
SELECT powerplants.primary_source,chicago_bgs.B02001002 FROM powerplants JOIN chicago_bgs ON ST_Within(powerplants.geom, chicago_bgs.geom);
