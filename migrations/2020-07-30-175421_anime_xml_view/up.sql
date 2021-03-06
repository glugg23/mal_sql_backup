CREATE VIEW anime_xml (mal_id, xml)
AS
SELECT a.mal_id,
'<anime>
    <series_animedb_id>' || a.mal_id || '</series_animedb_id>
    <series_title><![CDATA[' || title || ']]></series_title>
    <series_type>' || anime_type || '</series_type>
    <series_episodes>' || total_episodes || '</series_episodes>
    <my_id>0</my_id>
    <my_watched_episodes>' || watched_episodes || '</my_watched_episodes>
    <my_start_date>' || COALESCE(DATE(watch_start_date), '0000-00-00') || '</my_start_date>
    <my_finish_date>' || COALESCE(DATE(watch_end_date), '0000-00-00') || '</my_finish_date>
    <my_rated></my_rated>
    <my_score>' || score || '</my_score>
    <my_dvd></my_dvd>
    <my_storage>' || COALESCE(store.description, '') || '</my_storage>
    <my_status>' || anime_status || '</my_status>
    <my_comments><![CDATA[' || COALESCE(comments, '') || ']]></my_comments>
    <my_times_watched>' || COALESCE(times_rewatched, 0) || '</my_times_watched>
    <my_rewatch_value>' || COALESCE(rv.description, '') || '</my_rewatch_value>
    <my_tags><![CDATA[' || COALESCE(tags, '') || ']]></my_tags>
    <my_rewatching>' || is_rewatching || '</my_rewatching>
    <my_rewatching_ep>0</my_rewatching_ep>
    <update_on_import>1</update_on_import>
</anime>'
FROM anime a
LEFT JOIN anime_details ad ON a.mal_id = ad.mal_id
LEFT JOIN anime_storage store ON ad.storage = store.id
LEFT JOIN repeat_value rv on ad.rewatch_value = rv.id
JOIN statuses stats ON a.watching_status = stats.id;
