<div style="font-family:serif; text-align: justify;">

# Liger: How To


<img src="icon.png" width="300">

## Folder structure:

Your root folder should contain three directories (at some point I'd like to figure our how to make it so that you can just type "liger initialize" and it makes them for you).
- final
- layouts
- parts

THEY MUST BE NAMED EXACTLY AS WRITTEN.

"final" is untouched by you. Here, your compiled files will appear.

"layout" is where you will put your pages (index.html etc.) as well as any images or stylesheets. This will look like any other HTML file structure.

"parts" contains the building blocks that you want to frequently use such as a common header or footer.

## File structure:

For the most part, files are just as they would be for any other website, with one exception:
To inject external HTML into your document, use the "ligerplace" tag with a colon and the local file path from the parts folder like this:

<p>&ltligerplace:/header&gt </p>

## Compilation:

Running Liger from the command line in the root folder containing the _final_, _layouts_, and _parts_ folders will transfer all files from the _layouts_ folder into the _final_ folder, injecting the HTML where appropriate.

</div>