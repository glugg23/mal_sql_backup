CREATE VIEW user_anime_xml (user_id, xml)
AS
SELECT user_id,
'<myinfo>
    <user_id>' || user_id || '</user_id>
    <user_name>' || username || '</user_name>
    <user_export_type>1</user_export_type>
    <user_total_anime>' || anime_total_entries || '</user_total_anime>
    <user_total_watching>' || anime_watching || '</user_total_watching>
    <user_total_completed>' || anime_completed || '</user_total_completed>
    <user_total_onhold>' || anime_on_hold || '</user_total_onhold>
    <user_total_dropped>' || anime_dropped || '</user_total_dropped>
    <user_total_plantowatch>' || anime_plan_to_watch || '</user_total_plantowatch>
</myinfo>'
FROM users;

CREATE VIEW user_manga_xml (user_id, xml)
AS
SELECT user_id,
'<myinfo>
    <user_id>' || user_id || '</user_id>
    <user_name>' || username || '</user_name>
    <user_export_type>2</user_export_type>
    <user_total_manga>' || manga_total_entries || '</user_total_manga>
    <user_total_reading>' || manga_reading || '</user_total_reading>
    <user_total_completed>' || manga_completed || '</user_total_completed>
    <user_total_onhold>' || manga_on_hold || '</user_total_onhold>
    <user_total_dropped>' || manga_dropped || '</user_total_dropped>
    <user_total_plantoread>' || manga_plan_to_read || '</user_total_plantoread>
</myinfo>'
FROM users;
