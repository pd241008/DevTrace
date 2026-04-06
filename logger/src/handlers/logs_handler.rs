struct LofFilter{
    method: Option<Method>
    status: Option<u16>
    sort: Option<String>
}
 

pub fn build_filter(query_map):
/*
    if "method" exists:
        convert string → Method enum

    if "status" exists:
        parse to u16

    if "sort" exists:
        store as string

    return LogFilter

    */

    