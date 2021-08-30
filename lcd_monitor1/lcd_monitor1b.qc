$modelname	"props_j0w03l\lcd_monitor1b.mdl"
$body mybody	"lcd_monitor1b.smd"
$staticprop
$surfaceprop	combine_metal
$cdmaterials	"models\props_j0w03l\lcd_monitor1"

$sequence idle	"lcd_monitor1b.smd" // no animation wanted, so re-using the reference mesh

$collisionmodel	"lcd_monitor1b.smd" { $concave } // Temporary collision.

$texturegroup "screen"
{
	{ "lcd_greenscreen1" "lcd_plastic6" "lcd_plastic4" } //Skin 0 (default)
	{ "lcd_greenscreen1" "lcd_screen_border1" "lcd_screen_border1" } //Skin 1 (default)
	{ "lcd_screen1" "lcd_plastic6" "lcd_plastic4" } //Skin 2
	{ "lcd_screen1" "lcd_screen_border1" "lcd_screen_border1" } //Skin 3
	{ "lcd_screen2" "lcd_plastic6" "lcd_plastic4" } //Skin 4
	{ "lcd_screen2" "lcd_screen_border1" "lcd_screen_border1" } //Skin 5
	{ "lcd_screen3" "lcd_plastic6" "lcd_plastic4" } //Skin 6
	{ "lcd_screen3" "lcd_screen_border1" "lcd_screen_border1" } //Skin 6
	{ "lcd_screen4" "lcd_plastic6" "lcd_plastic4" } //Skin 7
	{ "lcd_screen4" "lcd_screen_border1" "lcd_screen_border1" } //Skin 7
	{ "lcd_screen5" "lcd_plastic6" "lcd_plastic4" } //Skin 10
	{ "lcd_screen5" "lcd_screen_border1" "lcd_screen_border1" } //Skin 11
}

$bodygroup "breathing"
{
	blank
	studio "lcd_monitor1b_breathing.smd"
}