
With Multiple Children - They Center as a GROUP
.container {
    display: flex;
    justify-content: center;
    align-items: center;
}
What happens:

All children are treated as one group
The entire group gets centered
Children maintain their natural spacing within the group
Visual Examples
Multiple Items in Row Direction
flex-direction: row; /* default */
justify-content: center;
align-items: center;
Result:

┌─────────────────────────────────┐
│                                 │
│        [A] [B] [C]              │ ← All centered as group
│                                 │
└─────────────────────────────────┘
Multiple Items in Column Direction
flex-direction: column;
justify-content: center;
align-items: center;
Result:

┌─────────────────────────────────┐
│                                 │
│             [A]                 │ ← All centered as group
│             [B]                 │
│             [C]                 │
│                                 │
└─────────────────────────────────┘
Real Example from Our Code
In our .canvas-container:

.canvas-container {
    display: flex;
    flex-direction: column;
    align-items: center;
}
Children:

<h1> (title)
<div> (grid)
Result:

┌─────────────────────────────────┐
│    Collaborative Pixel Canvas   │ ← Title centered horizontally
│                                 │
│         ████████████            │ ← Grid centered horizontally
│         ████████████            │
│         ████████████            │
└─────────────────────────────────┘
Both children are individually centered horizontally, but they stack vertically due to flex-direction: column.

Different justify-content Values with Multiple Children
justify-content: space-between
┌─────────────────────────────────┐
│ [A]        [B]        [C]       │ ← Equal space between
└─────────────────────────────────┘
justify-content: space-around
┌─────────────────────────────────┐
│   [A]      [B]      [C]         │ ← Equal space around each
└─────────────────────────────────┘
justify-content: flex-start
┌─────────────────────────────────┐
│ [A] [B] [C]                     │ ← All pushed to start
└─────────────────────────────────┘
When to Use Center vs Other Values
Use center when:
You want the group centered (single or multiple items)
Perfect for navigation menus, button groups, etc.
Use other values when:
space-between: Spread items across full width
flex-start/flex-end: Push all items to one side
space-around: Equal spacing around each item
Summary
justify-content: center and align-items: center work great with both single and multiple children. They center the entire group as one unit, which is usually what you want for layouts like ours!