Get-ChildItem .\* -Include "*.frag","*.vert" | Foreach {
    $outname = -join($_.name, ".spv")
    glslc -g -O $_.name -o $outname
    echo $outname
}