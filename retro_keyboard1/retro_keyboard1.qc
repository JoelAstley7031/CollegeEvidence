$modelname	"props_j0w03l\retro_keyboard1.mdl"
$body mybody	"retro_keyboard1.smd"
$staticprop
$surfaceprop	computer
$cdmaterials	"models\props_j0w03l\retro_keyboard1"

$sequence idle	"retro_keyboard1.smd" // no animation wanted, so re-using the reference mesh

$collisionmodel	"retro_keyboard1_SHADOW.smd" { $concave } // Re-use our shadow model.

$texturegroup "logo"
{
	{ "retro_keyboard1_logo1" } //Skin 0 (default)
	{ "retro_keyboard1_logo1_invis" } //Skin 1
}

// LOD 1
$lod 25
{
	replacemodel "retro_keyboard1.smd" "retro_keyboard1_LOD1.smd"
	replacematerial "retro_keyboard1_plastic1_keys" "retro_keyboard1_plastic1_LOD"
	replacematerial "retro_keyboard1_plastic2" "retro_keyboard1_plastic2_LOD"
}
// LOD 2
$lod 50
{
	replacemodel "retro_keyboard1.smd" "retro_keyboard1_LOD2.smd"
	replacematerial "retro_keyboard1_plastic1" "retro_keyboard1_plastic1_LOD"
	replacematerial "retro_keyboard1_plastic2" "retro_keyboard1_plastic2_LOD"
}
// LOD 3
$lod 75
{
	replacemodel "retro_keyboard1.smd" "retro_keyboard1_LOD3.smd"
	replacematerial "retro_keyboard1_plastic1" "retro_keyboard1_plastic1_LOD"
	replacematerial "retro_keyboard1_plastic2" "retro_keyboard1_plastic2_LOD"
}
// Model to render shadows with
$shadowlod
{
	replacemodel "retro_keyboard1.smd" "retro_keyboard1_SHADOW.smd"
}