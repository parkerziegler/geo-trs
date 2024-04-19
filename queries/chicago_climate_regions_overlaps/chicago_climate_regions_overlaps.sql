INSTALL spatial;
LOAD spatial;
CREATE TABLE acs_chicago_bgs AS SELECT * FROM ST_Read('datasets/acs-chicago-bgs.geojson');
CREATE TABLE climate_impact_regions AS SELECT * FROM ST_Read('datasets/wapo-climate-impact-regions.json');
SELECT climate_impact_regions.years_2020_2039, acs_chicago_bgs.B02001002 FROM climate_impact_regions JOIN acs_chicago_bgs ON ST_Intersects(climate_impact_regions.geom, acs_chicago_bgs.geom) AND ST_Intersects(acs_chicago_bgs.geom, climate_impact_regions.geom);
