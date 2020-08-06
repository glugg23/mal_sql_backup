CREATE VIEW manga_xml (mal_id, xml)
AS
SELECT m.mal_id,
'<manga>
    <manga_mangadb_id>' || m.mal_id || '</manga_mangadb_id>
    <manga_title><![CDATA[' || title || ']]></manga_title>
    <manga_volumes>' || total_volumes || '</manga_volumes>
    <manga_chapters>' || total_chapters || '</manga_chapters>
    <my_id>0</my_id>
    <my_read_volumes>' || read_volumes || '</my_read_volumes>
    <my_read_chapters>' || read_chapters || '</my_read_chapters>
    <my_start_date>' || COALESCE(DATE(read_start_date), '0000-00-00') || '</my_start_date>
    <my_finish_date>' || COALESCE(DATE(read_end_date), '0000-00-00') || '</my_finish_date>
    <my_scanalation_group><![CDATA[]]></my_scanalation_group>
    <my_score>' || score || '</my_score>
    <my_storage></my_storage>
    <my_status>' || manga_status || '</my_status>
    <my_comments><![CDATA[]]></my_comments>
    <my_times_read>' || md.times_reread || '</my_times_read>
    <my_tags><![CDATA[' || COALESCE(tags, '') || ']]></my_tags>
    <my_reread_value>' || COALESCE(rv.description, '') || '</my_reread_value>
    <update_on_import>1</update_on_import>
</manga>'
FROM manga m
LEFT JOIN manga_details md on m.mal_id = md.mal_id
LEFT JOIN repeat_value rv on md.reread_value = rv.id
JOIN statuses status ON reading_status = status.id;
