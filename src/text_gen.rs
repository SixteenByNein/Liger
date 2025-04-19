//Generates final text from a given file path

use std::fs;



pub fn text_gen(current_dir: String, root: String) -> String{



    let text_in = fs::read_to_string(current_dir.to_owned()).unwrap();


    let mut text_out: String = "".to_string();

    for item in slice_stream(text_in)
    {

        let tag = read_tag(item, root.clone());

        let tag_stream = tag.chars();

        for c in tag_stream
        {

            text_out.push(c);

        }

    }


    return text_out;

}


fn slice_stream(stream: String) -> Vec<String>
{

    let mut buffer = "".to_string();

    let mut master: Vec<String> = [].to_vec();

    for item in stream.chars()
    {

        if item == '<'
        {

            master.push(buffer.clone());

            buffer.clear();

            buffer.push(item);
            

        }
        else if item == '>'
        {

            buffer.push(item);

            master.push(buffer.clone());

            buffer.clear();

        }
        else
        {

            buffer.push(item);

        }

    }

    return master;

}

fn read_tag(tag: String, root: String) -> String
{

    if is_tag(tag.clone()) && split_tag(tag.clone())[0] == "ligerplace"
    {

        return fetch_text(split_tag(tag)[1].to_string(), root);

    }else {

        return tag;

    }

}


fn is_tag(tag: String) -> bool
{

    let mut char_stream: Vec<char> = [].to_vec();

    for c in tag.chars()
    {

        char_stream.push(c);

    };


    if !char_stream.is_empty() {

    return char_stream[0] == '<';

    } else {

        return false;
    
    }

}


fn split_tag(tag: String) -> [String; 2]
{

    let tag_descaled = descale(tag);

    let mut array_plate: [String; 2] = ["".to_string(),"".to_string()];

    let mut buffer: String = "".to_string();

    for c in tag_descaled.chars()
    {

        if c == ':'{

        array_plate[0] = buffer.clone();
        buffer.clear();

        } else {
            
            buffer.push(c);

        }

    }

    array_plate[1] = buffer;
    

    return array_plate;

}


fn fetch_text(path: String, root: String) -> String
{

    return fs::read_to_string(root + "/parts" + &path).unwrap()

}

fn descale(tag: String) -> String
{

    let mut complete: String = "".to_string();

    for c in tag.chars()
    {

        if ! (c == '<' || c == '>' || c == ' ')
        {
            complete.push(c);
        }

    }

    return complete;
    
}