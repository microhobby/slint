# Widgets

Widgets are not imported by default, and need to be imported from `"std-widgets.slint"`

Their appearance can change depending on the style

## `Button`

### Properties

* **`text`** (*string*): The text written in the button.
* **`icon`** (*image*): The image to show in the button. Note that not all styles support drawing icons.
* **`pressed`**: (*bool*): Set to true when the button is pressed.
* **`enabled`**: (*bool*): Defaults to true. When false, the button cannot be pressed

### Callbacks

* **`clicked`**

### Example

```slint
import { Button, VerticalBox } from "std-widgets.slint";
Example := Window {
    VerticalBox {
        Button {
            text: "Click Me";
            clicked => { self.text = "Clicked"; }
        }
    }
}
```

## `StandardButton`

The StandardButton looks like a button, but instead of customizing with `text` and `icon`,
it can used one of the pre-defined `kind` and the text and icon will depend on the style.

### Properties

* **`kind`** (*enum*): The kind of button, one of
   `ok` `cancel`, `apply`, `close`, `reset`, `help`, `yes`, `no,` `abort`, `retry` or `ignore`
* **`enabled`**: (*bool*): Defaults to true. When false, the button cannot be pressed

### Callbacks

* **`clicked`**

### Example

```slint
import { StandardButton, VerticalBox } from "std-widgets.slint";
Example := Window {
  VerticalBox {
    StandardButton { kind: ok; }
    StandardButton { kind: apply; }
    StandardButton { kind: cancel; }
  }
}
```

## `CheckBox`

### Properties

* **`text`** (*string*): The text written next to the checkbox.
* **`checked`**: (*bool*): Whether the checkbox is checked or not.

### Callbacks

* **`toggled`**: The checkbox value changed

### Example

```slint
import { CheckBox } from "std-widgets.slint";
Example := Window {
    width: 200px;
    height: 25px;
    CheckBox {
        width: parent.width;
        height: parent.height;
        text: "Hello World";
    }
}
```

## `SpinBox`

### Properties

* **`value`** (*int*): The value.
* **`minimum`** (*int*): The minimum value (default: 0).
* **`maximum`** (*int*): The maximum value (default: 100).

### Example

```slint
import { SpinBox } from "std-widgets.slint";
Example := Window {
    width: 200px;
    height: 25px;
    SpinBox {
        width: parent.width;
        height: parent.height;
        value: 42;
    }
}
```

## `Slider`

### Properties

* **`value`** (*float*): The value.
* **`minimum`** (*float*): The minimum value (default: 0)
* **`maximum`** (*float*): The maximum value (default: 100)

### Callbacks

* **`changed(float)`**: The value was changed

### Example

```slint
import { Slider } from "std-widgets.slint";
Example := Window {
    width: 200px;
    height: 25px;
    Slider {
        width: parent.width;
        height: parent.height;
        value: 42;
    }
}
```

## `GroupBox`

### Properties

* **`title`** (*string*): A text written as the title of the group box.

### Example

```slint
import { GroupBox } from "std-widgets.slint";
Example := Window {
    width: 200px;
    height: 100px;
    GroupBox {
        title: "A Nice Title";
        Text {
            text: "Hello World";
            color: blue;
        }
    }
}
```

## `LineEdit`

A widget used to enter a single line of text

### Properties

* **`text`** (*string*): The text being edited
* **`font-size`** (*length*): the size of the font of the input text
* **`has-focus`**: (*bool*): Set to true when the line edit currently has the focus
* **`placeholder-text`**: (*string*): A placeholder text being shown when there is no text in the edit field
* **`enabled`**: (*bool*): Defaults to true. When false, nothing can be entered
* **`read-only`** (bool): When set to `true`, text editing via keyboard and mouse is disabled but selecting text is still enabled as well as 
  editing text programatically (default value: `false`)
* **`input-type`** (*enum [`InputType`](#InputType)*): The way to allow special input viewing properties such as password fields (default value: `text`).
* **`horizontal-alignment`** (*enum [`TextHorizontalAlignment`](builtin_enums.md#texthorizontalalignment)*): The horizontal alignment of the text.

### Callbacks

* **`accepted`**: Enter was pressed
* **`edited`**: Emitted when the text has changed because the user modified it

### Example

```slint
import { LineEdit } from "std-widgets.slint";
Example := Window {
    width: 200px;
    height: 25px;
    LineEdit {
        font-size: 14px;
        width: parent.width;
        height: parent.height;
        placeholder-text: "Enter text here";
    }
}
```

## `TextEdit`

Similar to LineEdit, but can be used to enter several lines of text

*Note:* The current implementation only implement very few basic shortcut. More
shortcut will be implemented in a future version: <https://github.com/slint-ui/slint/issues/474>

### Properties

* **`text`** (*string*): The text being edited
* **`font-size`** (*length*): the size of the font of the input text
* **`has-focus`**: (*bool*): Set to true when the widget currently has the focus
* **`enabled`**: (*bool*): Defaults to true. When false, nothing can be entered
* **`read-only`** (bool): When set to `true`, text editing via keyboard and mouse is disabled but selecting text is still enabled as well as 
  editing text programatically (default value: `false`)
* **`wrap`** (*enum [`TextWrap`](builtin_enums.md#textwrap)*): The way the text wraps (default: word-wrap).
* **`horizontal-alignment`** (*enum [`TextHorizontalAlignment`](builtin_enums.md#texthorizontalalignment)*): The horizontal alignment of the text.

### Callbacks

* **`edited`**: Emitted when the text has changed because the user modified it

### Example

```slint
import { TextEdit } from "std-widgets.slint";
Example := Window {
    width: 200px;
    height: 200px;
    TextEdit {
        font-size: 14px;
        width: parent.width;
        height: parent.height;
        text: "Lorem ipsum dolor sit amet\n, consectetur adipisici elit";
    }
}
```


## `ScrollView`

A Scrollview contains a viewport that is bigger than the view and can be
scrolled. It has scrollbar to interact with. The viewport-width and
viewport-height are calculated automatically to create a scollable view
except for when using a for loop to populate the elements. In that case
the viewport-width and viewport-height are not calculated automatically
and must be set manually for scrolling to work. The ability to
automatically calculate the viewport-width and viewport-height when
using for loops may be added in the future and is tracked in issue #407.

### Properties

* **`viewport-width`** and **`viewport-height`** (*length*): The `width` and `length` properties of the viewport
* **`viewport-x`** and **`viewport-y`** (*length*): The `x` and `y` properties of the viewport. Usually these are negative
* **`visible-width`** and **`visible-height`** (*length*): The size of the visible area of the ScrollView (not including the scrollbar)
* **`enabled`** and **`has-focus`** (bool): property that are only used to render the frame as disabled or focused, but do not
  change the behavior of the widget.

### Example

```slint
import { ScrollView } from "std-widgets.slint";
Example := Window {
    width: 200px;
    height: 200px;
    ScrollView {
        width: 200px;
        height: 200px;
        viewport-width: 300px;
        viewport-height: 300px;
        Rectangle { width: 30px; height: 30px; x: 275px; y: 50px; background: blue; }
        Rectangle { width: 30px; height: 30px; x: 175px; y: 130px; background: red; }
        Rectangle { width: 30px; height: 30px; x: 25px; y: 210px; background: yellow; }
        Rectangle { width: 30px; height: 30px; x: 98px; y: 55px; background: orange; }
    }
}
```

## `ListView`

A ListView is like a Scrollview but it should have a `for` element, and the content are
automatically layed out in a list.
Elements are only instantiated if they are visible

### Properties

Same as ScrollView

### Example

```slint
import { ListView } from "std-widgets.slint";
Example := Window {
    width: 150px;
    height: 150px;
    ListView {
        width: 150px;
        height: 150px;
        for data in [
            { text: "Blue", color: #0000ff, bg: #eeeeee},
            { text: "Red", color: #ff0000, bg: #eeeeee},
            { text: "Green", color: #00ff00, bg: #eeeeee},
            { text: "Yellow", color: #ffff00, bg: #222222 },
            { text: "Black", color: #000000, bg: #eeeeee },
            { text: "White", color: #ffffff, bg: #222222 },
            { text: "Magenta", color: #ff00ff, bg: #eeeeee },
            { text: "Cyan", color: #00ffff, bg: #222222 },
        ] : Rectangle {
            height: 30px;
            background: data.bg;
            width: parent.width;
            Text {
                text: data.text;
                color: data.color;
            }
        }
    }
}
```

## `StandardListView`

Like ListView, but with a default delegate, and a `model` property which is a model of type
`StandardListViewItem`

The `StandardListViewItem` is equivalent to `{ text: string }` but will be improved in the future with `icon`, `checked` and so on (TODO)

### Properties

Same as ListView, and in addition:

* **`model`** (*`[StandardListViewItem]`*): The model
* **`current-item`** (*int*): The index of the currently active item. -1 mean none is selected, which is the default

### Example

```slint
import { StandardListView } from "std-widgets.slint";
Example := Window {
    width: 150px;
    height: 150px;
    StandardListView {
        width: 150px;
        height: 150px;
        model: [ { text: "Blue"}, { text: "Red" }, { text: "Green" },
            { text: "Yellow" }, { text: "Black"}, { text: "White"},
            { text: "Magenta" }, { text: "Cyan" },
        ];
    }
}
```

## `ComboBox`

A button that, when clicked, opens a popup to select a value.

### Properties

* **`model`** (*\[string\]*): The list of possible values
* **`current-index`**: (*int*): The index of the selected value (-1 if no value is selected)
* **`current-value`**: (*string*): The currently selected text
* **`enabled`**: (*bool*): When false, the combobox cannot be opened (default: true)

### Callbacks

* **`selected(string)`**: A value was selected from the combo box. The argument is the currently selected value.

### Example

```slint
import { ComboBox } from "std-widgets.slint";
Example := Window {
    width: 200px;
    height: 130px;
    ComboBox {
        width: preferred-width;
        height: preferred-height;
        model: ["first", "second", "third"];
        current-value: "first";
    }
}
```

## `TabWidget`

TabWidget is a container for a set of tabs. It can only have `Tab` elements as children and only one tab will be visible at
a time.

### Properties

* **`current-index`** (*int*): The index of the currently visible tab

### Properties of the `Tab` element

* **`title`** (*string*): The text written in the tab bar.

### Example

```slint
import { TabWidget } from "std-widgets.slint";
Example := Window {
    width: 200px;
    height: 200px;
    TabWidget {
        Tab {
            title: "First";
            Rectangle { background: orange; }
        }
        Tab {
            title: "Second";
            Rectangle { background: pink; }
        }
    }
}
```



## `HorizontalBox`, `VerticalBox`, `GridBox`

That's the same as `HorizontalLayout`, `VerticalLayout` or `GridLayout` but the spacing and padding values
 depending on the style instead of defaulting to 0.

## `AboutSlint`

This element displays the a "Made with Slint" badge.

```slint
import { AboutSlint } from "std-widgets.slint";
Example := Window {
    width: 128px;
    height: 128px;
    AboutSlint {
    }
}
```