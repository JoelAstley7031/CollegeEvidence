$modelname	"props_j0w03l\lcd_monitor1.mdl"
$body mybody	"lcd_monitor1.smd"
$staticprop
$surfaceprop	combine_metal
$cdmaterials	"models\props_j0w03l\lcd_monitor1"

$sequence idle	"lcd_monitor1.smd" // no animation wanted, so re-using the reference mesh

$collisionmodel	"lcd_monitor1.smd" { $concave } // Temporary collision.

$texturegroup "screen"
{
	{ "lcd_greenscreen1" } //Skin 0 (default)
	{ "lcd_screen1"     } //Skin 1
	{ "lcd_screen2"     } //Skin 2
	{ "lcd_screen3"     } //Skin 3
	{ "lcd_screen4"     } //Skin 4
	{ "lcd_screen5"     } //Skin 5
}
