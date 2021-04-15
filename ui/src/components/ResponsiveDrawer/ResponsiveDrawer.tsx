import React from 'react';

//Material UI imports
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import IconButton from '@material-ui/core/IconButton';
import MenuIcon from '@material-ui/icons/Menu';
import { makeStyles, Theme, createStyles } from '@material-ui/core/styles';
import Divider from '@material-ui/core/Divider';
import Drawer from '@material-ui/core/Drawer';
import Hidden from '@material-ui/core/Hidden';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListItemIcon from '@material-ui/core/ListItemIcon';
import ListItemText from '@material-ui/core/ListItemText';
import CssBaseline from '@material-ui/core/CssBaseline';
import SaveIcon from '@material-ui/icons/Save';
import SettingsIcon from '@material-ui/icons/Settings';
import GetAppIcon from '@material-ui/icons/GetApp';
import VideogameAssetIcon from '@material-ui/icons/VideogameAsset';
import KeyboardIcon from '@material-ui/icons/Keyboard';
import Switch from '@material-ui/core/Switch';
import DeleteIcon from '@material-ui/icons/Delete';



const drawerWidth = 240;

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      display: 'flex',
    },
    drawer: {
      [theme.breakpoints.up('sm')]: {
        width: drawerWidth,
        flexShrink: 0,
      },
    },
    appBar: {
      [theme.breakpoints.up('sm')]: {
        zIndex: theme.zIndex.drawer + 1,
      },
    },
    menuButton: {
      marginRight: theme.spacing(2),
      [theme.breakpoints.up('sm')]: {
        display: 'none',
      },
    },
    // necessary for content to be below app bar
    toolbar: theme.mixins.toolbar,
    drawerPaper: {
      width: drawerWidth,
    },
    content: {
      flexGrow: 1,
      padding: theme.spacing(3),
    },
  }),
);

function MenuItem (props: any) {
    let {text, onClick} = props;
    return (
        <ListItem onClick={onClick} button key={text}>
          <ListItemIcon>
            {props.children}
          </ListItemIcon>
          <ListItemText primary={text} />
        </ListItem>
  
    )
}
  
function ResponsiveDrawer(props: any) {
const classes = useStyles();
const [mobileOpen, setMobileOpen] = React.useState(false);

const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
};

let w: any = window;
const drawerContent = (
    <div>
    <div className={classes.toolbar} />
    <Divider />
    <List>
        <MenuItem onClick={() => w.emu.load_save_state()} text={"Load"}><GetAppIcon/></MenuItem>
        <MenuItem onClick={() => w.emu.make_save_state()} text={"Save"}><SaveIcon/></MenuItem>
        <ListItem button key={"Mute"}>
        <ListItemIcon>
            <SettingsIcon/>
        </ListItemIcon>
        <ListItemText primary={"Mute"} />
        <Switch
            checked={!props.togglemute}
            color="default"
            onChange={props.onMute}
        />
        </ListItem>
        <ListItem button key={"Configure GamePad"}>
        <ListItemIcon>
            <VideogameAssetIcon/>
        </ListItemIcon>
        <ListItemText primary={"Configure GamePad"} />
        <Switch
            checked={!props.toggle}
            color="default"
            onChange={props.onGamepadChange}
        />
        </ListItem>
        <MenuItem onClick={props.onKeyboardChange} text={"Configure Keyboard"}><KeyboardIcon/> {props.modal} </MenuItem>
        <MenuItem text={"Delete User Data"} onClick={props.onDelete}><DeleteIcon/></MenuItem>
    </List>
    </div>
);

return (
    <div className={classes.root}>
    <CssBaseline />
    <AppBar position="fixed" className={classes.appBar}>
        <Toolbar>
        <IconButton
            color="inherit"
            aria-label="open drawer"
            edge="start"
            onClick={handleDrawerToggle}
            className={classes.menuButton}
        >
            <MenuIcon />
        </IconButton>
        <Typography variant="h6" noWrap>
            {props.name}
        </Typography>
        </Toolbar>
    </AppBar>
    <nav className={classes.drawer} aria-label="mailbox folders">
        <Hidden smUp implementation="css">
        <Drawer
            variant="temporary"
            anchor='left'
            open={mobileOpen}
            onClose={handleDrawerToggle}
            classes={{
            paper: classes.drawerPaper,
            }}
            ModalProps={{
            keepMounted: true, // Better open performance on mobile.
            }}
        >
            {drawerContent}
        </Drawer>
        </Hidden>
        <Hidden xsDown implementation="css">
        <Drawer
            classes={{
            paper: classes.drawerPaper,
            }}
            variant="permanent"
            open
        >
            {drawerContent}
        </Drawer>
        </Hidden>
    </nav>
    <main className={classes.content}>
    </main>
    </div>
);

}

export default ResponsiveDrawer;