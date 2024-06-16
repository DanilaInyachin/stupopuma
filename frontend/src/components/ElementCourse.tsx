import React, { FC, useContext, useEffect, useState } from 'react';
import {
  Box,
  List,
  ListItem,
  ListItemText,
  Collapse,
  Button,
  Typography,
} from '@mui/material';
import { ExpandLess, ExpandMore } from '@mui/icons-material';
import CurrentUserContext from '../contex';
import axios from 'axios';

interface ICourse {
  namecourses: string;
  id: number;
  nametheme: string;
}

interface ElementCourseProps {
  namecourse: string;
}

const ElementCourse: FC<ElementCourseProps> = ({ namecourse }) => {
  const [open, setOpen] = useState(false);
  const [courses, setCourses] = useState<ICourse[]>([]);
  const context = useContext(CurrentUserContext);

  useEffect(() => {
    if (context) {
      axios.post('//localhost:8080/get_topics_by_course', {
        namecourses: namecourse
      }).then((response) => {
        console.log(response);
        setCourses(response.data);
      }).catch((error) => {
        console.error('Error: ', error);
      })
    }
  }, [context, namecourse, setCourses]);

  const handleToggle = () => {
    setOpen((prevOpen) => !prevOpen);
  };

  const handleEnroll = () => {
    // console.log('Enroll to course ID:', course.id);
    // Логика записи на курс
  };

  return (
    <Box>
      <ListItem onClick={handleToggle}>
        <ListItemText primary={namecourse} />
        {open ? <ExpandLess /> : <ExpandMore />}
      </ListItem>
      <Collapse in={open} timeout="auto" unmountOnExit>
        <List component="div" disablePadding>
          {courses.map((course) => (
            <ListItem key={course.id} sx={{ pl: 4 }}>
              <ListItemText primary={course.nametheme} />
            </ListItem>
          ))}
        </List>
        <Box sx={{ display: 'flex', justifyContent: 'flex-end', p: 2 }}>
          <Button variant="contained" color="primary" onClick={handleEnroll}>
            Enroll
          </Button>
        </Box>
      </Collapse>
    </Box>
  );
};

export default ElementCourse;