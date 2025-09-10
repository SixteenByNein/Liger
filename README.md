<div style="font-family:serif; text-align: justify;">

# Liger: How To


<img src="icon.png" width="300">

## Folder structure:

to initialize your directory, run "liger init"

Your root folder should contain three directories:

- final
- layout
- parts

as well as a "template.html" and an "index.html."

"final" and "index.html are untouched by you. Here, your compiled files will appear.

"layout" is where you will put your pages (index.html etc.) as well as any images or stylesheets. This will look like any other HTML file structure.

"template.html" will be compiled into the "index.html."

"parts" contains the building blocks that you want to frequently use such as a common header or footer.

Further folders can be added, these will neither be edited, nor compiled from. As there will be no deleting nor copying from these directories, they are a great place to put any images or other large files that do not need to be duplicated.

This root directory _is the root directory of your site_ when referencing a file in relation to the root such as "/style.css," it will be in relatiion to this root directory.

## File structure:

For the most part, files are just as they would be for any other website, with one exception:
To inject external HTML into your document, use the "ligerplace" tag with a colon and the local file path from the parts folder like this:

<p>&ltligerplace:/header&gt </p>

## Compilation:

Running Liger from the command line in the root folder containing the _final_, _layouts_, and _parts_ folders will transfer all files from the _layouts_ folder into the _final_ folder, injecting the HTML where appropriate.

</div>