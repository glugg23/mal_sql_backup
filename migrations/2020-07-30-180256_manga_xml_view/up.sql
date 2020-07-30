CREATE VIEW manga_xml (mal_id, xml)
AS
SELECT mal_id,
'<manga>
    <manga_mangadb_id>' || mal_id || '</manga_mangadb_id>
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
    <my_times_read>0</my_times_read>
    <my_tags><![CDATA[' || COALESCE(tags, '') || ']]></my_tags>
    <my_reread_value></my_reread_value>
    <update_on_import>1</update_on_import>
</manga>'
FROM manga
JOIN statuses ON reading_status = id;
