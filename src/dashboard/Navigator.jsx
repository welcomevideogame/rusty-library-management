import * as React from 'react';
import Divider from '@mui/material/Divider';
import Drawer from '@mui/material/Drawer';
import List from '@mui/material/List';
import Box from '@mui/material/Box';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemIcon from '@mui/material/ListItemIcon';
import ListItemText from '@mui/material/ListItemText';
import HomeIcon from '@mui/icons-material/Home';
import DnsRoundedIcon from '@mui/icons-material/DnsRounded';
import PermMediaOutlinedIcon from '@mui/icons-material/PhotoSizeSelectActual';
import PublicIcon from '@mui/icons-material/Public';
import SettingsEthernetIcon from '@mui/icons-material/SettingsEthernet';
import SettingsInputComponentIcon from '@mui/icons-material/SettingsInputComponent';
import TimerIcon from '@mui/icons-material/Timer';
import SettingsIcon from '@mui/icons-material/Settings';
import PhonelinkSetupIcon from '@mui/icons-material/PhonelinkSetup';

const categories = [
  {
    id: 'System',
    children: [
      { id: 'Items', icon: <DnsRoundedIcon />, permission: 'User' },
      { id: 'Employees', icon: <PermMediaOutlinedIcon />, permission: 'Admin' },
      { id: 'Placeholder2', icon: <PublicIcon />, permission: 'User' },
      { id: 'Placeholder3', icon: <SettingsEthernetIcon />, permission: 'Admin' },
      { id: 'Placeholder4', icon: <SettingsInputComponentIcon />, permission: 'Admin' },
    ],
  },
  {
    id: 'System',
    children: [
      { id: 'Settings', icon: <SettingsIcon />, permission: 'User' },
    ],
  },
  {
    id: 'Developer Tools',
    children: [
      { id: 'Analytics', icon: <SettingsIcon />, permission: 'Manager' },
      { id: 'Performance', icon: <TimerIcon />, permission: 'Dev'},
      { id: 'Test Lab', icon: <PhonelinkSetupIcon />, permission: 'Dev'},
    ],
  },
];

const item = {
  py: '2px',
  px: 3,
  color: 'rgba(255, 255, 255, 0.7)',
  '&:hover, &:focus': {
    bgcolor: 'rgba(255, 255, 255, 0.08)',
  },
};

const itemCategory = {
  boxShadow: '0 -1px 0 rgb(255,255,255,0.1) inset',
  py: 1.5,
  px: 3,
};

export default function Navigator(props) {
  const { rank, ...other } = props;

  // Check if the user has the required rank to view an item
  const hasPermission = (requiredRank) => {
    const rankHierarchy = ['None', 'Basic', 'User', 'Manager', 'Admin', 'Dev'];
    return rankHierarchy.indexOf(rank) >= rankHierarchy.indexOf(requiredRank);
  };

  return (
    <Drawer variant="permanent" {...other}>
      <List disablePadding>
        {categories.map(({ id, children }) => (
          <Box key={id} sx={{ bgcolor: '#101F33' }}>
            <ListItem sx={{ py: 2, px: 3 }}>
              <ListItemText sx={{ color: '#fff' }}>{id}</ListItemText>
            </ListItem>
            {children
              .filter(child => hasPermission(child.permission))
              .map(({ id: childId, icon }) => (
                <ListItem disablePadding key={childId}>
                  <ListItemButton sx={item}>
                    <ListItemIcon>{icon}</ListItemIcon>
                    <ListItemText>{childId}</ListItemText>
                  </ListItemButton>
                </ListItem>
            ))}
            <Divider sx={{ mt: 2 }} />
          </Box>
        ))}
      </List>
    </Drawer>
  );
}