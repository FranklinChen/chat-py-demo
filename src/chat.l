%%

<INITIAL>@([ \t\r\n]+|[ \t]+|(\r\n|\r|\n)\t)* "HEADER"
<INITIAL>\*([ \t\r\n]+|[ \t]+|(\r\n|\r|\n)\t)* "MAIN_TIER"
<INITIAL>%([ \t\r\n]+|[ \t]+|(\r\n|\r|\n)\t)* "DEPENDENT_TIER"

\r\n|\r|\n <INITIAL>"NEWLINE"

. "UNMATCHED"
