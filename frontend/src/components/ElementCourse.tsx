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
import CurrentUserContext from '../context';
import axios from 'axios';

interface ICourse {
  namecourses: string;
  id: number;
  nametheme: string;
}

interface ElementCourseProps {
  namecourse: string;
  needButton?: boolean;
  isTeacher?: boolean;
}

const ElementCourse: FC<ElementCourseProps> = ({
  namecourse,
  needButton = true,
  isTeacher = false,
}) => {
  const [open, setOpen] = useState(false);
  const [courses, setCourses] = useState<ICourse[]>([]);
  const context = useContext(CurrentUserContext);

  useEffect(() => {
    if (context) {
      axios
        .post('//localhost:8080/get_topics_by_course', {
          namecourses: namecourse,
        })
        .then((response) => {
          console.log(response);
          setCourses(response.data);
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
    }
  }, [context, namecourse, setCourses]);

  const handleToggle = () => {
    setOpen((prevOpen) => !prevOpen);
  };

  const handleEnroll = () => {
    if (context && context.isAuthAndToken) {
      axios
        .post('//localhost:8080/register_user_courses', {
          nameCourses: namecourse,
          token: context.isAuthAndToken,
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
    }
  };

  const handleEditCourse = () => {
    console.log('Edit course');
  };

  const handleAddTopic = () => {
    console.log('Add topic');
  };

  const handleEditTopic = () => {
    console.log('Edit topic');
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
        {needButton ? (
          <Box sx={{ display: 'flex', justifyContent: 'flex-end', p: 2 }}>
            <Button variant="contained" color="primary" onClick={handleEnroll}>
              Enroll
            </Button>
          </Box>
        ) : null}
        {isTeacher && (
          <Box sx={{ display: 'flex', justifyContent: 'space-between', p: 2 }}>
            <Button
              variant="outlined"
              color="primary"
              onClick={handleEditCourse}
            >
              Изменить курс
            </Button>
            <Button variant="outlined" color="primary" onClick={handleAddTopic}>
              Добавить тему
            </Button>
            <Button
              variant="outlined"
              color="primary"
              onClick={handleEditTopic}
            >
              Изменить тему
            </Button>
          </Box>
        )}
      </Collapse>
    </Box>
  );
};

export default ElementCourse;
