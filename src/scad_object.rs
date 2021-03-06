use crate::ScadElement;

/**
    An scad object which is a single scad element and can have zero or more child objects

    #How it works
    An scad object is a single `ScadElement` optionally followed by any number of child
    objects. This represents the following scad code:

    ```SCAD
    translate([1,2,3]) //parent
    {
        cube([3,5,1]); //Child
        //...
    }
    ```

    Without using the `scad!` macro, you would create an scad object by doing the
    following.

    ```
    # use scad::*;
    //Create the parent
    let mut obj = ScadObject::new(ScadElement::Union);

    //add some children
    obj.add_child(ScadObject::new(ScadElement::Cube(vec3(1., 1., 1.))));
    //...
    ```

    This would be quite tedious to type each time you want to create a new object
    which is why the `scad!` macro exists. This does mean that if you want to add
    more children to an scad object created by the macro, you can simply use the
    `add_child` function on the result of the macro.
*/
#[derive(Clone)]
pub struct ScadObject {
    element: ScadElement,

    children: Vec<ScadObject>,

    //Decides whether or not the object should be drawn alone (by adding ! before)
    important: bool,

    // TODO - document modifier precedence and update the tests
    // '#'
    highlight: bool,

    // '%'
    transparent: bool,
}

impl ScadObject {
    pub fn new(element: ScadElement) -> ScadObject {
        ScadObject {
            element,
            children: Vec::new(),
            important: false,
            highlight: false,
            transparent: false,
        }
    }

    pub fn add_child(&mut self, statement: ScadObject) {
        self.children.push(statement);
    }

    /**
      Returns the scad code for the object.

      If there are no children, only the code for the ScadElement of the
      object followed by a `;` is returned. If children exist, the code for
      the element is returned first, followed by the code for each child surrounded
      by `{}` and indented 1 tab character.
    */
    pub fn get_code(&self) -> String {
        let mut result: String;

        //Get the code for the current element
        result = self.element.clone().get_code();

        if self.important {
            result = String::from("!") + &result;
        } else if self.highlight {
            result = String::from("#") + &result;
        } else if self.transparent {
            result = String::from("%") + &result;
        }

        //Adding the code for all children, or ; if none exist
        result = result
            + &(match self.children.len() {
                0 => String::from(";"),
                _ => {
                    let mut child_code = String::from("\n{\n");
                    for stmt in &self.children {
                        //Add the children indented one line
                        child_code = child_code + "\t" + &(stmt.get_code().replace("\n", "\n\t"));
                        child_code += "\n";
                    }

                    //Add the final bracket and 'return' the result
                    child_code + "}"
                }
            });

        result
    }

    pub fn is_important(&self) -> bool {
        self.important
    }

    pub fn is_highlighted(&self) -> bool {
        self.highlight
    }

    pub fn is_transparent(&self) -> bool {
        self.transparent
    }

    /**
      Marks the object as important. This will prepend the object code
      with an ! which tells scad to only render that object and its children.
    */
    pub fn set_important(&mut self) {
        self.important = true;
        self.highlight = false;
        self.transparent = false;
    }

    pub fn set_highlighted(&mut self) {
        self.important = false;
        self.highlight = true;
        self.transparent = false;
    }

    pub fn set_transparent(&mut self) {
        self.important = false;
        self.highlight = false;
        self.transparent = true;
    }

    /**
      Takes ownership over the object, marks it as important and returns it.
      Usefull if you want to mark something as important without having to
      change the binding to mut
    */
    pub fn important(mut self) -> ScadObject {
        self.important = true;
        self.transparent = false;
        self.highlight = false;
        self
    }

    pub fn highlight(mut self) -> ScadObject {
        self.important = false;
        self.highlight = true;
        self.transparent = false;
        self
    }

    pub fn transparent(mut self) -> ScadObject {
        self.important = false;
        self.highlight = false;
        self.transparent = true;
        self
    }
}

#[cfg(test)]
mod statement_tests {
    use super::*;
    use crate::na;

    #[test]
    fn simple_stmt_test() {
        let mut test_stmt =
            ScadObject::new(ScadElement::Translate(na::Vector3::new(0.0, 0.0, 0.0)));

        assert_eq!(test_stmt.get_code(), "translate([0,0,0]);");

        test_stmt.add_child(ScadObject::new(ScadElement::Cube(na::Vector3::new(
            1.0, 1.0, 1.0,
        ))));
        assert_eq!(
            test_stmt.get_code(),
            "translate([0,0,0])\n{\n\tcube([1,1,1]);\n}"
        );

        test_stmt.set_important();
        assert_eq!(
            test_stmt.get_code(),
            "!translate([0,0,0])\n{\n\tcube([1,1,1]);\n}"
        );

        test_stmt.set_highlighted();
        assert_eq!(
            test_stmt.get_code(),
            "#translate([0,0,0])\n{\n\tcube([1,1,1]);\n}"
        );

        test_stmt.set_transparent();
        assert_eq!(
            test_stmt.get_code(),
            "%translate([0,0,0])\n{\n\tcube([1,1,1]);\n}"
        );

        let test_2 = ScadObject::new(ScadElement::Union).important();
        assert_eq!(test_2.get_code(), "!union();");
    }
}
